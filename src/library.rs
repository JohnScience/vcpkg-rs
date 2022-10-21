use std::path::PathBuf;

/// Details of a package that was found
#[derive(Debug)]
pub struct Library {
    /// Paths for the linker to search for static or import libraries
    pub link_paths: Vec<PathBuf>,

    /// Paths to search at runtme to find DLLs
    pub dll_paths: Vec<PathBuf>,

    /// Paths to include files
    pub include_paths: Vec<PathBuf>,

    /// cargo: metadata lines
    pub cargo_metadata: Vec<String>,

    /// libraries found are static
    pub is_static: bool,

    /// DLLs found
    pub found_dlls: Vec<PathBuf>,

    /// static libs or import libs found
    pub found_libs: Vec<PathBuf>,

    /// link name of libraries found, this is useful to emit linker commands
    pub found_names: Vec<String>,

    /// ports that are providing the libraries to link to, in port link order
    pub ports: Vec<String>,

    /// the vcpkg triplet that has been selected
    pub vcpkg_triplet: String,
}

impl Library {
    // Should it be a public function?
    pub(crate) fn new(is_static: bool, vcpkg_triplet: &str) -> Library {
        Library {
            link_paths: Vec::new(),
            dll_paths: Vec::new(),
            include_paths: Vec::new(),
            cargo_metadata: Vec::new(),
            is_static,
            found_dlls: Vec::new(),
            found_libs: Vec::new(),
            found_names: Vec::new(),
            ports: Vec::new(),
            vcpkg_triplet: vcpkg_triplet.to_string(),
        }
    }
}
