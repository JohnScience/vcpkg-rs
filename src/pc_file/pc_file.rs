use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{Error, TargetTriplet, VcpkgTarget};

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

    pub(crate) fn from_str(
        id: &str,
        s: &str,
        target_triplet: &TargetTriplet,
    ) -> Result<Self, Error> {
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
