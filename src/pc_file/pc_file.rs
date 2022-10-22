use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{Error, TargetTriplet, VcpkgTarget};

/// Parsed knowledge from a .pc file.
///
/// Learn more about .pc files here:
/// * <https://manpages.ubuntu.com/manpages/focal/man5/pc.5.html>
/// * <https://linux.die.net/man/1/pkg-config>
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
    pub(crate) fn parse(vcpkg_target: &VcpkgTarget, path: &Path) -> Result<Self, Error> {
        // Extract the pkg-config name.
        let id = path
            .file_stem()
            .ok_or_else(|| {
                Error::VcpkgInstallation(format!(
                    "pkg-config file {} has bogus name",
                    path.to_string_lossy()
                ))
            })?
            .to_string_lossy();
        // Read through the file and gather what we want.
        let mut file = File::open(path)
            .map_err(|_| Error::VcpkgInstallation(format!("Couldn't open {}", path.display())))?;
        let mut pc_file_contents = String::new();

        file.read_to_string(&mut pc_file_contents)
            .map_err(|_| Error::VcpkgInstallation(format!("Couldn't read {}", path.display())))?;
        PcFile::from_str(&id, &pc_file_contents, &vcpkg_target.target_triplet)
    }

    pub(crate) fn from_str(
        id: &str,
        s: &str,
        target_triplet: &TargetTriplet,
    ) -> Result<Self, Error> {
        let mut libs = Vec::new();
        let mut deps = Vec::new();

        let preparsed_lines_iter = s
            .lines()
            .filter_map(|line| line.split_once(|c| c == ':'))
            // we defer the evaluation of split_whitespace() until we actually need it
            .map(|(prop_kw, remainder)| (prop_kw, move || remainder.split_whitespace()));

        // Read abour property keywords of .pc files here:
        // https://manpages.ubuntu.com/manpages/focal/man5/pc.5.html#:~:text=has%20been%20done.-,PROPERTY%20KEYWORDS,-Name%20%20%20%20The%20displayed
        for (prop_kw, split_remainder) in preparsed_lines_iter {
            // We could collect a lot of stuff here, but we only care about Requires and Libs for the moment.
            match prop_kw {
                "Requires" => {
                    let mut requires_args = split_remainder()
                        .flat_map(|e| e.split(","))
                        .filter(|s| !s.is_empty());
                    while let Some(dep) = requires_args.next() {
                        // Drop any versioning requirements, we only care about library order and rely upon
                        // port dependencies to resolve versioning.
                        if dep.contains(|c| c == '=' || c == '<' || c == '>') {
                            requires_args.next();
                            continue;
                        }
                        deps.push(dep.to_owned());
                    }
                }
                "Libs" => {
                    for lib_flag in split_remainder() {
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
                _ => continue,
            }
        }

        Ok(PcFile {
            id: id.to_string(),
            libs,
            deps,
        })
    }
}
