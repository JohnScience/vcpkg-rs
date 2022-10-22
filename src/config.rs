use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::env_vars::cargo::build_rs::OUT_DIR;
use crate::{
    envify, find_vcpkg_target, load_ports, msvc_target, remove_item, Error, Library, Port,
    TargetTriplet, VcpkgTarget,
};

/// Configuration options for finding packages, setting up the tree and emitting metadata to cargo
#[derive(Default)]
pub struct Config {
    /// should the cargo metadata actually be emitted
    pub(crate) cargo_metadata: bool,

    /// should cargo:include= metadata be emitted (defaults to false)
    pub(crate) emit_includes: bool,

    /// .lib/.a files that must be be found for probing to be considered successful
    pub(crate) required_libs: Vec<String>,

    /// .dlls that must be be found for probing to be considered successful
    pub(crate) required_dlls: Vec<String>,

    /// should DLLs be copied to OUT_DIR?
    pub(crate) copy_dlls: bool,

    /// override VCPKG_ROOT environment variable
    pub(crate) vcpkg_root: Option<PathBuf>,

    pub(crate) target: Option<TargetTriplet>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            cargo_metadata: true,
            copy_dlls: true,
            ..Default::default()
        }
    }

    fn get_target_triplet(&mut self) -> Result<TargetTriplet, Error> {
        use crate::env_vars::vcpkg_rs::VCPKGRS_TRIPLET;

        if self.target.is_none() {
            let target = if let Ok(triplet_str) = env::var(VCPKGRS_TRIPLET) {
                triplet_str.into()
            } else {
                msvc_target()?
            };
            self.target = Some(target);
        }

        Ok(self.target.as_ref().unwrap().clone())
    }

    /// Find the package `port_name` in a Vcpkg tree.
    ///
    /// Emits cargo metadata to link to libraries provided by the Vcpkg package/port
    /// named, and any (non-system) libraries that they depend on.
    ///
    /// This will select the architecture and linkage based on environment
    /// variables and build flags as described in the module docs, and any configuration
    /// set on the builder.
    pub fn find_package(&mut self, port_name: &str) -> Result<Library, Error> {
        use crate::env_vars::vcpkg_rs::prelude::*;

        // determine the target type, bailing out if it is not some
        // kind of msvc
        let msvc_target = self.get_target_triplet()?;

        // bail out if requested to not try at all
        if env::var_os(VCPKGRS_DISABLE).is_some() {
            return Err(Error::DisabledByEnv(VCPKGRS_DISABLE.to_owned()));
        }

        // bail out if requested to not try at all (old)
        if env::var_os(NO_VCPKG).is_some() {
            return Err(Error::DisabledByEnv(NO_VCPKG.to_owned()));
        }

        // bail out if requested to skip this package
        let abort_var_name = format!("{}{}", prefix::VCPKGRS_NO_, envify(port_name));
        if env::var_os(&abort_var_name).is_some() {
            return Err(Error::DisabledByEnv(abort_var_name));
        }

        // bail out if requested to skip this package (old)
        let abort_var_name = format!("{}{}", envify(port_name), suffix::_NO_VCPKG);
        if env::var_os(&abort_var_name).is_some() {
            return Err(Error::DisabledByEnv(abort_var_name));
        }

        let vcpkg_target = find_vcpkg_target(&self, &msvc_target)?;
        let mut required_port_order = Vec::new();

        // if no overrides have been selected, then the Vcpkg port name
        // is the the .lib name and the .dll name
        if self.required_libs.is_empty() {
            let ports = load_ports(&vcpkg_target)?;

            if ports.get(&port_name.to_owned()).is_none() {
                return Err(Error::LibNotFound(format!(
                    "package {} is not installed for vcpkg triplet {}",
                    port_name.to_owned(),
                    vcpkg_target.target_triplet.triplet
                )));
            }

            // the complete set of ports required
            let mut required_ports: BTreeMap<String, Port> = BTreeMap::new();
            // working of ports that we need to include
            //        let mut ports_to_scan: BTreeSet<String> = BTreeSet::new();
            //        ports_to_scan.insert(port_name.to_owned());
            let mut ports_to_scan = vec![port_name.to_owned()]; //: Vec<String> = BTreeSet::new();

            while !ports_to_scan.is_empty() {
                let port_name = ports_to_scan.pop().unwrap();

                if required_ports.contains_key(&port_name) {
                    continue;
                }

                if let Some(port) = ports.get(&port_name) {
                    for dep in &port.deps {
                        ports_to_scan.push(dep.clone());
                    }
                    required_ports.insert(port_name.clone(), (*port).clone());
                    remove_item(&mut required_port_order, &port_name);
                    required_port_order.push(port_name);
                } else {
                    // what?
                }
            }

            // for port in ports {
            //     println!("port {:?}", port);
            // }
            // println!("== Looking for port {}", port_name);
            // for port in &required_port_order {
            //     println!("ordered required port {:?}", port);
            // }
            // println!("=============================");
            // for port in &required_ports {
            //     println!("required port {:?}", port);
            // }

            // if no overrides have been selected, then the Vcpkg port name
            // is the the .lib name and the .dll name
            if self.required_libs.is_empty() {
                for port_name in &required_port_order {
                    let port = required_ports.get(port_name).unwrap();
                    self.required_libs.extend(port.libs.iter().map(|s| {
                        Path::new(&s)
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned()
                    }));
                    self.required_dlls
                        .extend(port.dlls.iter().cloned().map(|s| {
                            Path::new(&s)
                                .file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned()
                        }));
                }
            }
        }
        // require explicit opt-in before using dynamically linked
        // variants, otherwise cargo install of various things will
        // stop working if Vcpkg is installed.
        if !vcpkg_target.target_triplet.is_static && !env::var_os(VCPKGRS_DYNAMIC).is_some() {
            return Err(Error::RequiredEnvMissing(VCPKGRS_DYNAMIC.to_owned()));
        }

        let mut lib = Library::new(
            vcpkg_target.target_triplet.is_static,
            &vcpkg_target.target_triplet.triplet,
        );

        if self.emit_includes {
            lib.cargo_metadata.push(format!(
                "cargo:include={}",
                vcpkg_target.include_path.display()
            ));
        }
        lib.include_paths.push(vcpkg_target.include_path.clone());

        lib.cargo_metadata.push(format!(
            "cargo:rustc-link-search=native={}",
            vcpkg_target
                .lib_path
                .to_str()
                .expect("failed to convert string type")
        ));
        lib.link_paths.push(vcpkg_target.lib_path.clone());
        if !vcpkg_target.target_triplet.is_static {
            lib.cargo_metadata.push(format!(
                "cargo:rustc-link-search=native={}",
                vcpkg_target
                    .bin_path
                    .to_str()
                    .expect("failed to convert string type")
            ));
            // this path is dropped by recent versions of cargo hence the copies to OUT_DIR below
            lib.dll_paths.push(vcpkg_target.bin_path.clone());
        }

        lib.ports = required_port_order;

        self.emit_libs(&mut lib, &vcpkg_target)?;

        if self.copy_dlls {
            self.do_dll_copy(&mut lib)?;
        }

        if self.cargo_metadata {
            for line in &lib.cargo_metadata {
                println!("{}", line);
            }
        }
        Ok(lib)
    }

    /// Define whether metadata should be emitted for cargo allowing it to
    /// automatically link the binary. Defaults to `true`.
    pub fn cargo_metadata(&mut self, cargo_metadata: bool) -> &mut Config {
        self.cargo_metadata = cargo_metadata;
        self
    }

    /// Define cargo:include= metadata should be emitted. Defaults to `false`.
    pub fn emit_includes(&mut self, emit_includes: bool) -> &mut Config {
        self.emit_includes = emit_includes;
        self
    }

    /// Should DLLs be copied to OUT_DIR?
    /// Defaults to `true`.
    pub fn copy_dlls(&mut self, copy_dlls: bool) -> &mut Config {
        self.copy_dlls = copy_dlls;
        self
    }

    /// Define which path to use as vcpkg root overriding the VCPKG_ROOT environment variable
    /// Default to `None`, which means use VCPKG_ROOT or try to find out automatically
    pub fn vcpkg_root(&mut self, vcpkg_root: PathBuf) -> &mut Config {
        self.vcpkg_root = Some(vcpkg_root);
        self
    }

    /// Specify target triplet. When triplet is not specified, inferred triplet from rust target is used.
    ///
    /// Specifying a triplet using `target_triplet` will override the default triplet for this crate. This
    /// cannot change the choice of triplet made by other crates, so a safer choice will be to set
    /// `VCPKGRS_TRIPLET` in the environment which will allow all crates to use a consistent set of
    /// external dependencies.
    pub fn target_triplet<S: AsRef<str>>(&mut self, triplet: S) -> &mut Config {
        self.target = Some(triplet.into());
        self
    }

    /// Find the library `port_name` in a Vcpkg tree.
    ///
    /// This will use all configuration previously set to select the
    /// architecture and linkage.
    /// Deprecated in favor of the find_package function
    #[doc(hidden)]
    pub fn probe(&mut self, port_name: &str) -> Result<Library, Error> {
        use crate::env_vars::vcpkg_rs::prelude::*;

        // determine the target type, bailing out if it is not some
        // kind of msvc
        let msvc_target = self.get_target_triplet()?;

        // bail out if requested to not try at all
        if env::var_os(VCPKGRS_DISABLE).is_some() {
            return Err(Error::DisabledByEnv(VCPKGRS_DISABLE.to_owned()));
        }

        // bail out if requested to not try at all (old)
        if env::var_os(NO_VCPKG).is_some() {
            return Err(Error::DisabledByEnv(NO_VCPKG.to_owned()));
        }

        // bail out if requested to skip this package
        let abort_var_name = format!("{}{}", prefix::VCPKGRS_NO_, envify(port_name));
        if env::var_os(&abort_var_name).is_some() {
            return Err(Error::DisabledByEnv(abort_var_name));
        }

        // bail out if requested to skip this package (old)
        let abort_var_name = format!("{}{}", envify(port_name), suffix::_NO_VCPKG);
        if env::var_os(&abort_var_name).is_some() {
            return Err(Error::DisabledByEnv(abort_var_name));
        }

        // if no overrides have been selected, then the Vcpkg port name
        // is the the .lib name and the .dll name
        if self.required_libs.is_empty() {
            self.required_libs.push(port_name.to_owned());
            self.required_dlls.push(port_name.to_owned());
        }

        let vcpkg_target = find_vcpkg_target(&self, &msvc_target)?;

        // require explicit opt-in before using dynamically linked
        // variants, otherwise cargo install of various things will
        // stop working if Vcpkg is installed.
        if !vcpkg_target.target_triplet.is_static && !env::var_os(VCPKGRS_DYNAMIC).is_some() {
            return Err(Error::RequiredEnvMissing(VCPKGRS_DYNAMIC.to_owned()));
        }

        let mut lib = Library::new(
            vcpkg_target.target_triplet.is_static,
            &vcpkg_target.target_triplet.triplet,
        );

        if self.emit_includes {
            lib.cargo_metadata.push(format!(
                "cargo:include={}",
                vcpkg_target.include_path.display()
            ));
        }
        lib.include_paths.push(vcpkg_target.include_path.clone());

        lib.cargo_metadata.push(format!(
            "cargo:rustc-link-search=native={}",
            vcpkg_target
                .lib_path
                .to_str()
                .expect("failed to convert string type")
        ));
        lib.link_paths.push(vcpkg_target.lib_path.clone());
        if !vcpkg_target.target_triplet.is_static {
            lib.cargo_metadata.push(format!(
                "cargo:rustc-link-search=native={}",
                vcpkg_target
                    .bin_path
                    .to_str()
                    .expect("failed to convert string type")
            ));
            // this path is dropped by recent versions of cargo hence the copies to OUT_DIR below
            lib.dll_paths.push(vcpkg_target.bin_path.clone());
        }

        self.emit_libs(&mut lib, &vcpkg_target)?;

        if self.copy_dlls {
            self.do_dll_copy(&mut lib)?;
        }

        if self.cargo_metadata {
            for line in &lib.cargo_metadata {
                println!("{}", line);
            }
        }
        Ok(lib)
    }

    fn emit_libs(&mut self, lib: &mut Library, vcpkg_target: &VcpkgTarget) -> Result<(), Error> {
        for required_lib in &self.required_libs {
            // this could use static-nobundle= for static libraries but it is apparently
            // not necessary to make the distinction for windows-msvc.

            let link_name = match vcpkg_target.target_triplet.strip_lib_prefix {
                true => required_lib.trim_left_matches("lib"),
                false => required_lib,
            };

            lib.cargo_metadata
                .push(format!("cargo:rustc-link-lib={}", link_name));

            lib.found_names.push(String::from(link_name));

            // verify that the library exists
            let mut lib_location = vcpkg_target.lib_path.clone();
            lib_location.push(required_lib.clone() + "." + &vcpkg_target.target_triplet.lib_suffix);

            if !lib_location.exists() {
                return Err(Error::LibNotFound(lib_location.display().to_string()));
            }
            lib.found_libs.push(lib_location);
        }

        if !vcpkg_target.target_triplet.is_static {
            for required_dll in &self.required_dlls {
                let mut dll_location = vcpkg_target.bin_path.clone();
                dll_location.push(required_dll.clone() + ".dll");

                // verify that the DLL exists
                if !dll_location.exists() {
                    return Err(Error::LibNotFound(dll_location.display().to_string()));
                }
                lib.found_dlls.push(dll_location);
            }
        }

        Ok(())
    }

    fn do_dll_copy(&mut self, lib: &mut Library) -> Result<(), Error> {
        if let Some(target_dir) = env::var_os(OUT_DIR) {
            if !lib.found_dlls.is_empty() {
                for file in &lib.found_dlls {
                    let mut dest_path = Path::new(target_dir.as_os_str()).to_path_buf();
                    dest_path.push(Path::new(file.file_name().unwrap()));

                    fs::copy(file, &dest_path).map_err(|_| {
                        Error::LibNotFound(format!(
                            "Can't copy file {} to {}",
                            file.to_string_lossy(),
                            dest_path.to_string_lossy()
                        ))
                    })?;
                    println!(
                        "vcpkg build helper copied {} to {}",
                        file.to_string_lossy(),
                        dest_path.to_string_lossy()
                    );
                }
                lib.cargo_metadata.push(format!(
                    "cargo:rustc-link-search=native={}",
                    env::var(OUT_DIR).unwrap()
                ));
                // work around https://github.com/rust-lang/cargo/issues/3957
                lib.cargo_metadata.push(format!(
                    "cargo:rustc-link-search={}",
                    env::var(OUT_DIR).unwrap()
                ));
            }
        } else {
            return Err(Error::LibNotFound(format!("Unable to get {}", OUT_DIR)));
        }
        Ok(())
    }

    /// Override the name of the library to look for if it differs from the package name.
    ///
    /// It should not be necessary to use `lib_name` anymore. Calling `find_package` with a package name
    /// will result in the correct library names.
    /// This may be called more than once if multiple libs are required.
    /// All libs must be found for the probe to succeed. `.probe()` must
    /// be run with a different configuration to look for libraries under one of several names.
    /// `.libname("ssleay32")` will look for ssleay32.lib and also ssleay32.dll if
    /// dynamic linking is selected.
    pub fn lib_name(&mut self, lib_stem: &str) -> &mut Config {
        self.required_libs.push(lib_stem.to_owned());
        self.required_dlls.push(lib_stem.to_owned());
        self
    }

    /// Override the name of the library to look for if it differs from the package name.
    ///
    /// It should not be necessary to use `lib_names` anymore. Calling `find_package` with a package name
    /// will result in the correct library names.
    /// This may be called more than once if multiple libs are required.
    /// All libs must be found for the probe to succeed. `.probe()` must
    /// be run with a different configuration to look for libraries under one of several names.
    /// `.lib_names("libcurl_imp","curl")` will look for libcurl_imp.lib and also curl.dll if
    /// dynamic linking is selected.
    pub fn lib_names(&mut self, lib_stem: &str, dll_stem: &str) -> &mut Config {
        self.required_libs.push(lib_stem.to_owned());
        self.required_dlls.push(dll_stem.to_owned());
        self
    }
}
