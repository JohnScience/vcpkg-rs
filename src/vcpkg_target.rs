use std::path::PathBuf;

use crate::TargetTriplet;

/// paths and triple for the chosen target
pub(crate) struct VcpkgTarget {
    pub(crate) lib_path: PathBuf,
    pub(crate) bin_path: PathBuf,
    pub(crate) include_path: PathBuf,

    // directory containing the status file
    pub(crate) status_path: PathBuf,
    // directory containing the install files per port.
    pub(crate) packages_path: PathBuf,

    // target-specific settings.
    pub(crate) target_triplet: TargetTriplet,
}

impl VcpkgTarget {
    pub(crate) fn link_name_for_lib(&self, filename: &std::path::Path) -> Option<String> {
        if self.target_triplet.strip_lib_prefix {
            filename.to_str().map(|s| s.to_owned())
        // filename
        //     .to_str()
        //     .map(|s| s.trim_left_matches("lib").to_owned())
        } else {
            filename.to_str().map(|s| s.to_owned())
        }
    }
}
