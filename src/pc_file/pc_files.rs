use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use super::PcFile;
use crate::{remove_item, Error, VcpkgTarget};

/// Collection of [`PcFile`]s.  Can be built and queried as a set of .pc files.
#[derive(Debug)]
pub(crate) struct PcFiles {
    pub(crate) files: HashMap<String, PcFile>,
}

impl PcFiles {
    pub(crate) fn load_pkgconfig_dir(
        vcpkg_target: &VcpkgTarget,
        path: &PathBuf,
    ) -> Result<Self, Error> {
        let mut files = HashMap::new();
        for dir_entry in path.read_dir().map_err(|e| {
            Error::VcpkgInstallation(format!(
                "Missing pkgconfig directory {}: {}",
                path.to_string_lossy(),
                e
            ))
        })? {
            let dir_entry = dir_entry.map_err(|e| {
                Error::VcpkgInstallation(format!(
                    "Troubling reading pkgconfig dir {}: {}",
                    path.to_string_lossy(),
                    e
                ))
            })?;
            // Only look at .pc files.
            if dir_entry.path().extension() != Some(OsStr::new("pc")) {
                continue;
            }
            let pc_file = PcFile::parse(vcpkg_target, &dir_entry.path())?;
            files.insert(pc_file.id.to_owned(), pc_file);
        }
        Ok(PcFiles { files })
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
