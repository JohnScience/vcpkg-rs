#[derive(Clone)]
pub(crate) struct TargetTriplet {
    pub(crate) vcpkg_triplet: String,
    pub(crate) is_static: bool,
    pub(crate) lib_suffix: String,
    pub(crate) strip_lib_prefix: bool,
}

impl<S: AsRef<str>> From<S> for TargetTriplet {
    fn from(triplet: S) -> TargetTriplet {
        let triplet = triplet.as_ref();
        if triplet.contains("windows") {
            TargetTriplet {
                vcpkg_triplet: triplet.into(),
                is_static: triplet.contains("-static"),
                lib_suffix: "lib".into(),
                strip_lib_prefix: false,
            }
        } else {
            TargetTriplet {
                vcpkg_triplet: triplet.into(),
                is_static: true,
                lib_suffix: "a".into(),
                strip_lib_prefix: true,
            }
        }
    }
}
