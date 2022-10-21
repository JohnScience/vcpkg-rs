// should these environment variables be public?

pub(crate) const VCPKGRS_TRIPLET: &'static str = "VCPKGRS_TRIPLET";
pub(crate) const VCPKGRS_DISABLE: &'static str = "VCPKGRS_DISABLE";

#[cfg(any(test, doctest))]
pub(crate) const ARBITRARY_VCPKGRS_NO_FOO: &'static str = "VCPKGRS_NO_FOO";