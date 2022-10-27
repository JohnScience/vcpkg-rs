mod rustc_support_tier;
mod arch;
mod sub;

#[derive(Clone)]
pub(crate) struct VcpkgTriplet {
    pub(crate) name: String,
    pub(crate) is_static: bool,
    pub(crate) lib_suffix: String,
    pub(crate) strip_lib_prefix: bool,
}

impl VcpkgTriplet {
    const NON_WINDOWS_LIB_SUFFIX: &'static str = "a";
    const WINDOWS_LIB_SUFFIX: &'static str = "lib";
}

impl<S: AsRef<str>> From<S> for VcpkgTriplet
{
    fn from(triplet: S) -> VcpkgTriplet {
        let triplet = triplet.as_ref();
        if triplet.contains("windows") {
            VcpkgTriplet {
                name: triplet.into(),
                is_static: triplet.contains("-static"),
                lib_suffix: "lib".into(),
                strip_lib_prefix: false,
            }
        } else {
            VcpkgTriplet {
                name: triplet.into(),
                is_static: true,
                lib_suffix: "a".into(),
                strip_lib_prefix: true,
            }
        }
    }
}
