pub(super) struct Sub {
    #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
    pub(super) llvm_name: Option<&'static str>,
    pub(super) vcpkg_name: Option<&'static str>,
    pub(super) rustc_name: Option<&'static str>,
}

impl Sub {
    pub(super) const ARM_V4T: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v4t"),
    };

    pub(super) const ARM_V5TE: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v5te"),
    };

    pub(super) const ARM_V6: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v6"),
    };

    pub(super) const ARM_V6K: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v6k"),
    };

    pub(super) const ARM_V7: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7"),
    };

    pub(super) const ARM_V7A: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7a"),
    };

    pub(super) const ARM_V7K: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7k"),
    };

    pub(super) const ARM_V7R: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7r"),
    };

    pub(super) const ARM_V7S: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7s"),
    };

    pub(super) const ARMEB_V7R: Self = Self::ARM_V7R;

    pub(super) const RISCV32_GC: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("gc"),
    };

    pub(super) const RISCV32_I: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("i"),
    };

    pub(super) const RISCV32_IM: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("im"),
    };

    pub(super) const RISCV32_IMAC: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("imac"),
    };

    pub(super) const RISCV32_IMC: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("imc"),
    };

    pub(super) const RISCV64_GC: Self = Self::RISCV32_GC;

    pub(super) const RISCV64_IMAC: Self = Self::RISCV32_IMAC;

    pub(super) const THUMB_V4T: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v4t"),
    };

    pub(super) const THUMB_V6M: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v6m"),
    };

    pub(super) const THUMB_V7A: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7a"),
    };

    pub(super) const THUMB_V7EM: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7em"),
    };

    pub(super) const THUMB_V7M: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7m"),
    };

    pub(super) const THUMB_V7NEON: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v7neon"),
    };

    pub(super) const THUMB_V8M: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("v8m"),
    };
}