use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::ffi::OsStr;

use crate::{VcpkgTarget, TargetTriplet, Error, remove_item};

/// Parsed knowledge from a .pc file.
#[derive(Debug)]
pub(crate) struct PcFile {
    /// The pkg-config name of this library.
    pub(crate) id: String,
    /// List of libraries found as '-l', translated to a given vcpkg_target. e.g. libbrotlicommon.a
    pub(crate) libs: Vec<String>,
    /// List of pkgconfig dependencies, e.g. PcFile::id.
    pub(crate) deps: Vec<String>,
}

impl PcFile {
    pub(crate) fn parse_pc_file(vcpkg_target: &VcpkgTarget, path: &Path) -> Result<Self, Error> {
        // Extract the pkg-config name.
        let id = try!(path
            .file_stem()
            .ok_or_else(|| Error::VcpkgInstallation(format!(
                "pkg-config file {} has bogus name",
                path.to_string_lossy()
            ))))
        .to_string_lossy();
        // Read through the file and gather what we want.
        let mut file = try!(File::open(path)
            .map_err(|_| Error::VcpkgInstallation(format!("Couldn't open {}", path.display()))));
        let mut pc_file_contents = String::new();

        try!(file
            .read_to_string(&mut pc_file_contents)
            .map_err(|_| Error::VcpkgInstallation(format!("Couldn't read {}", path.display()))));
        PcFile::from_str(&id, &pc_file_contents, &vcpkg_target.target_triplet)
    }

    pub(crate) fn from_str(id: &str, s: &str, target_triplet: &TargetTriplet) -> Result<Self, Error> {
        let mut libs = Vec::new();
        let mut deps = Vec::new();

        for line in s.lines() {
            // We could collect a lot of stuff here, but we only care about Requires and Libs for the moment.
            if line.starts_with("Requires:") {
                let mut requires_args = line
                    .split(":")
                    .skip(1)
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .flat_map(|e| e.split(","))
                    .filter(|s| *s != "");
                while let Some(dep) = requires_args.next() {
                    // Drop any versioning requirements, we only care about library order and rely upon
                    // port dependencies to resolve versioning.
                    if let Some(_) = dep.find(|c| c == '=' || c == '<' || c == '>') {
                        requires_args.next();
                        continue;
                    }
                    deps.push(dep.to_owned());
                }
            } else if line.starts_with("Libs:") {
                let lib_flags = line
                    .split(":")
                    .skip(1)
                    .next()
                    .unwrap_or("")
                    .split_whitespace();
                for lib_flag in lib_flags {
                    if lib_flag.starts_with("-l") {
                        // reconstruct the library name.
                        let lib = format!(
                            "{}{}.{}",
                            if target_triplet.strip_lib_prefix {
                                "lib"
                            } else {
                                ""
                            },
                            lib_flag.trim_left_matches("-l"),
                            target_triplet.lib_suffix
                        );
                        libs.push(lib);
                    }
                }
            }
        }

        Ok(PcFile {
            id: id.to_string(),
            libs,
            deps,
        })
    }
}

/// Collection of PcFile.  Can be built and queried as a set of .pc files.
#[derive(Debug)]
pub(crate) struct PcFiles {
    pub(crate) files: HashMap<String, PcFile>,
}

impl PcFiles {
    pub(crate) fn load_pkgconfig_dir(vcpkg_target: &VcpkgTarget, path: &PathBuf) -> Result<Self, Error> {
        let mut files = HashMap::new();
        for dir_entry in try!(path.read_dir().map_err(|e| {
            Error::VcpkgInstallation(format!(
                "Missing pkgconfig directory {}: {}",
                path.to_string_lossy(),
                e
            ))
        })) {
            let dir_entry = try!(dir_entry.map_err(|e| {
                Error::VcpkgInstallation(format!(
                    "Troubling reading pkgconfig dir {}: {}",
                    path.to_string_lossy(),
                    e
                ))
            }));
            // Only look at .pc files.
            if dir_entry.path().extension() != Some(OsStr::new("pc")) {
                continue;
            }
            let pc_file = try!(PcFile::parse_pc_file(vcpkg_target, &dir_entry.path()));
            files.insert(pc_file.id.to_owned(), pc_file);
        }
        Ok(PcFiles { files: files })
    }

    /// Use the .pc files as a hint to the library sort order.
    pub(crate) fn fix_ordering(&self, mut libs: Vec<String>) -> Vec<String> {
        // Overall heuristic: for each library given as input, identify which PcFile declared it.
        // Then, looking at that PcFile, check its Requires: (deps), and if the pc file for that
        // dep is in our set, check if its libraries are in our set of libs.  If so, move it to the
        // end to ensure it gets linked afterwards.

        // We may need to do this a few times to properly handle the case where A -> (depends on) B
        // -> C -> D and libraries were originally sorted D, C, B, A.  Avoid recursion so we don't
        // have to detect potential cycles.
        for _iter in 0..3 {
            let mut required_lib_order: Vec<String> = Vec::new();
            for lib in &libs {
                required_lib_order.push(lib.to_owned());
                if let Some(pc_file) = self.locate_pc_file_by_lib(lib) {
                    // Consider its requirements:
                    for dep in &pc_file.deps {
                        // Only consider pkgconfig dependencies we know about.
                        if let Some(dep_pc_file) = self.files.get(dep) {
                            // Intra-port library ordering found, pivot any already seen dep_lib to the
                            // end of the list.
                            for dep_lib in &dep_pc_file.libs {
                                if let Some(removed) = remove_item(&mut required_lib_order, dep_lib)
                                {
                                    required_lib_order.push(removed);
                                }
                            }
                        }
                    }
                }
            }
            // We should always end up with the same number of libraries, only their order should
            // change.
            assert_eq!(libs.len(), required_lib_order.len());
            // Termination:
            if required_lib_order == libs {
                // Nothing changed, we're done here.
                return libs;
            }
            libs = required_lib_order;
        }
        println!("cargo:warning=vcpkg gave up trying to resolve pkg-config ordering.");
        libs
    }
    /// Locate which PcFile contains this library, if any.
    pub(crate) fn locate_pc_file_by_lib(&self, lib: &str) -> Option<&PcFile> {
        for (id, pc_file) in &self.files {
            if pc_file.libs.contains(&lib.to_owned()) {
                return Some(pc_file);
            }
        }
        None
    }
}
