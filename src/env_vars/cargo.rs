/// This module contains the environment variables that are being used by
/// this crate and are [set by Cargo for build scripts].
///
/// [variables that Cargo sets for build scripts]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
pub(crate) mod build_rs {

    /// The [`TARGET`] environment variable which is [set by Cargo for build scripts].
    /// Also, it is the target triple that the crate using `vcpkg-rs` is being compiled for.
    /// Native code should be compiled for this triple. See the [Target Triple] description for more information.
    ///
    /// [set by Cargo for build script]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    /// [`TARGET`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts:~:text=package%20in%20question.-,TARGET,-%E2%80%94%20the%20target%20triple
    pub(crate) const TARGET: &'static str = "TARGET";

    /// The [`OUT_DIR`] environment variable which is [set by Cargo for build scripts].
    /// Also, it is the folder in which all output and intermediate artifacts should be placed.
    /// This folder is inside the build directory for the package being built,
    /// and it is unique for the package in question.
    /// 
    /// [set by Cargo for build script]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    /// [`OUT_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts:~:text=target%20features%20enabled.-,OUT_DIR,-%E2%80%94%20the%20folder%20in
    pub(crate) const OUT_DIR: &'static str = "OUT_DIR";
}
