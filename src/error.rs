use std::error;
use std::fmt;

#[derive(Debug)] // need Display?
pub enum Error {
    /// Aborted because of a `VCPKGRS_NO_*` environment variable.
    ///
    /// Contains the name of the responsible environment variable.
    DisabledByEnv(String),

    /// Aborted because a required environment variable was not set.
    RequiredEnvMissing(String),

    /// On Windows, only MSVC ABI is supported
    NotMSVC,

    /// Can't find a vcpkg tree
    VcpkgNotFound(String),

    /// Library not found in vcpkg tree
    LibNotFound(String),

    /// Could not understand vcpkg installation
    VcpkgInstallation(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DisabledByEnv(_) => "vcpkg-rs requested to be aborted",
            Error::RequiredEnvMissing(_) => "a required env setting is missing",
            Error::NotMSVC => "vcpkg-rs only can only find libraries for MSVC ABI builds",
            Error::VcpkgNotFound(_) => "could not find Vcpkg tree",
            Error::LibNotFound(_) => "could not find library in Vcpkg tree",
            Error::VcpkgInstallation(_) => "could not look up details of packages in vcpkg tree",
            Error::__Nonexhaustive => panic!(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // Error::Command { ref cause, .. } => Some(cause),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::DisabledByEnv(ref name) => write!(f, "Aborted because {} is set", name),
            Error::RequiredEnvMissing(ref name) => write!(f, "Aborted because {} is not set", name),
            Error::NotMSVC => write!(
                f,
                "the vcpkg-rs Vcpkg build helper can only find libraries built for the MSVC ABI."
            ),
            Error::VcpkgNotFound(ref detail) => write!(f, "Could not find Vcpkg tree: {}", detail),
            Error::LibNotFound(ref detail) => {
                write!(f, "Could not find library in Vcpkg tree {}", detail)
            }
            Error::VcpkgInstallation(ref detail) => write!(
                f,
                "Could not look up details of packages in vcpkg tree {}",
                detail
            ),
            Error::__Nonexhaustive => panic!(),
        }
    }
}
