// should these environment variables be public?

pub(crate) const VCPKGRS_TRIPLET: &'static str = "VCPKGRS_TRIPLET";
pub(crate) const VCPKGRS_DISABLE: &'static str = "VCPKGRS_DISABLE";
pub(crate) const VCPKGRS_DYNAMIC: &'static str = "VCPKGRS_DYNAMIC";
pub(crate) const NO_VCPKG: &'static str = "NO_VCPKG";
pub(crate) const VCPKG_ROOT: &'static str = "VCPKG_ROOT";

#[cfg(any(test, doctest))]
pub(crate) const ARBITRARY_VCPKGRS_NO_FOO: &'static str = concat!("VCPKGRS_NO_", "FOO");

pub(crate) mod prefix {
    pub(crate) const VCPKGRS_NO_: &'static str = "VCPKGRS_NO_";
}

pub(crate) mod suffix {
    pub(crate) const _NO_VCPKG: &'static str = "_NO_VCPKG";
}

pub(crate) mod prelude {
    pub(crate) use super::*;
}
