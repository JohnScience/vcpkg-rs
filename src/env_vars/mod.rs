// should the module be public?
pub(crate) mod vcpkg_rs;

pub(crate) mod cargo;

pub(crate) mod prelude {
    pub(crate) use super::vcpkg_rs::prelude::*;
    pub(crate) use super::cargo::prelude::*;
}