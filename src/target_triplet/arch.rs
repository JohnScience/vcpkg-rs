/// Target architecture.
/// 
/// # Learn more
/// 
/// You can learn more names of different [target triple] components [here](https://github.com/llvm-mirror/llvm/blob/4586bdecf94a6f327c4b98d54a55fd8278003e27/include/llvm/ADT/Triple.h).
/// More complete list of [target triple] components is available [here](https://llvm.org/doxygen/classllvm_1_1Triple.html).
/// 
/// You can obtain the list of targets supported by [`rustc`] via `rustc --print target-list`.
/// 
/// You can obtain the list of targets supported by [`vcpkg`] via `vcpkg help triplets`.
/// 
/// # Notes
/// 
/// [Target triple is a misnomer][target triple].
/// 
/// Using `vcpkg help triplets | rg -o "^\s+[^-]+-" | huniq | rg -o "[^\s-]+"` (with [`vcpkg`], [`ripgrep`].
/// and [`huniq`] installed) will help you obtain the list of supported by [`vcpkg`] concatenated 
/// `<arch><sub>` pairs, e.g. `armv6` where `<arch>` is `arm` and `<sub>` is `v6`.
/// 
/// Similarly, using `rustc --print target-list | rg -o "^[^-]+-" | huniq | rg -o "^[^-]+"` (with [`vcpkg`],
/// [`riprep`], and [`huniq`] installed) will help you obtain the list of supported by [`rustc`] concatenated
/// `<arch><sub>` pairs, e.g. `armv6` where `<arch>` is `arm` and `<sub>` is `v6`.
/// 
/// Manual maintenance of this module is a **pain**.
/// 
/// [`vcpkg`]: https://vcpkg.readthedocs.io/en/latest/users/triplets/
/// [`LLVM`]: https://clang.llvm.org/docs/CrossCompilation.html
/// [`rustc`]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
/// [`ripgrep`]: https://crates.io/crates/ripgrep
/// [`huniq`]: https://crates.io/crates/huniq
/// [target triple]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
pub(super) struct Arch {
    #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
    pub(super) llvm_name: Option<&'static str>,
    pub(super) vcpkg_name: Option<&'static str>,
    pub(super) rustc_name: Option<&'static str>,
}

// Approach via macro rule makes compile times better by checking cfg only twice

#[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
macro_rules! decl_supported_only_by_llvm {
    () => {
        pub(super) const LLVM_ARC: Self = Self {
            llvm_name: Some("arc"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_CSKY: Self = Self {
            llvm_name: Some("csky"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_DXIL: Self = Self {
            llvm_name: Some("dxil"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_LOONGARCH32: Self = Self {
            llvm_name: Some("loongarch32"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_LOONGARCH64: Self = Self {
            llvm_name: Some("loongarch64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_M68K: Self = Self {
            llvm_name: Some("m68k"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_R600: Self = Self {
            llvm_name: Some("r600"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_AMDGCN: Self = Self {
            llvm_name: Some("amdgcn"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_SPARCEL: Self = Self {
            llvm_name: Some("sparcel"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_TCE: Self = Self {
            llvm_name: Some("tce"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_TCELE: Self = Self {
            llvm_name: Some("tcele"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_THUMBEB: Self = Self {
            llvm_name: Some("thumbeb"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_XCORE: Self = Self {
            llvm_name: Some("xcore"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_NVPTX: Self = Self {
            llvm_name: Some("nvptx"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_LE32: Self = Self {
            llvm_name: Some("le32"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_LE64: Self = Self {
            llvm_name: Some("le64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_AMDIL: Self = Self {
            llvm_name: Some("amdil"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_AMDIL64: Self = Self {
            llvm_name: Some("amdil64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_HSAIL: Self = Self {
            llvm_name: Some("hsail"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_HSAIL64: Self = Self {
            llvm_name: Some("hsail64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_SPIR: Self = Self {
            llvm_name: Some("spir"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_SPIR64: Self = Self {
            llvm_name: Some("spir64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_SPIRV32: Self = Self {
            llvm_name: Some("spirv32"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_SPIRV64: Self = Self {
            llvm_name: Some("spirv64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_KALIMBA: Self = Self {
            llvm_name: Some("kalimba"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_SHAVE: Self = Self {
            llvm_name: Some("shave"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_LANAI: Self = Self {
            llvm_name: Some("lanai"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_RENDERSCRIPT32: Self = Self {
            llvm_name: Some("renderscript32"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_RENDERSCRIPT64: Self = Self {
            llvm_name: Some("renderscript64"),
            vcpkg_name: None,
            rustc_name: None,
        };

        pub(super) const LLVM_VE: Self = Self {
            llvm_name: Some("ve"),
            vcpkg_name: None,
            rustc_name: None,
        };
    };
}

// The constants are ordered as in https://llvm.org/doxygen/classllvm_1_1Triple.html
impl Arch {
    /// # Vcpkg targets
    /// 
    /// * `arm-uwp`
    /// * `arm-android`
    /// * `arm-ios`
    /// * `arm-linux`
    /// * `arm-mingw-dynamic`
    /// * `arm-mingw-static`
    /// * `arm-neon-android`
    /// * `arm-windows-static`
    /// * `arm-windows`
    /// * `armv6-android`
    /// 
    /// # Rustc targets
    /// 
    /// * `arm-linux-androideabi`
    /// * `arm-unknown-linux-gnueabi`
    /// * `arm-unknown-linux-gnueabihf`
    /// * `arm-unknown-linux-musleabi`
    /// * `arm-unknown-linux-musleabihf`
    /// * `armv4t-unknown-linux-gnueabi`
    /// * `armv5te-unknown-linux-gnueabi`
    /// * `armv5te-unknown-linux-musleabi`
    /// * `armv5te-unknown-linux-uclibceabi`
    /// * `armv6-unknown-freebsd`
    /// * `armv6-unknown-netbsd-eabihf`
    /// * `armv6k-nintendo-3ds`
    /// * `armv7-apple-ios`
    /// * `armv7-linux-androideabi`
    /// * `armv7-unknown-freebsd`
    /// * `armv7-unknown-linux-gnueabi`
    /// * `armv7-unknown-linux-gnueabihf`
    /// * `armv7-unknown-linux-musleabi`
    /// * `armv7-unknown-linux-musleabihf`
    /// * `armv7-unknown-linux-uclibceabi`
    /// * `armv7-unknown-linux-uclibceabihf`
    /// * `armv7-unknown-netbsd-eabihf`
    /// * `armv7-wrs-vxworks-eabihf`
    /// * `armv7a-kmc-solid_asp3-eabi`
    /// * `armv7a-kmc-solid_asp3-eabihf`
    /// * `armv7a-none-eabi`
    /// * `armv7a-none-eabihf`
    /// * `armv7k-apple-watchos`
    /// * `armv7r-none-eabi`
    /// * `armv7r-none-eabihf`
    /// * `armv7s-apple-ios`
    /// 
    /// # Notes
    /// 
    /// [Target triple is a misnomer][target triple].
    /// 
    /// `v6`, `v4t`, `v5te`, `v6`, `v6k`, `v7`, `v7a`, `v7k`, `v7r`, `v7s` above are `<sub>`
    /// [target triple] components.
    /// 
    /// # Reproducibility
    /// 
    /// You can use
    /// 
    /// 1. `vcpkg help triplets | rg "^\s+arm(v[^-]+)?-"`
    /// 2. `rustc --print target-list | rg "^arm(v[^-]+)?-"`
    /// 
    /// to obtain the same or similar results.
    /// 
    /// [target triple]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
    pub(super) const ARM: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("arm"),
        vcpkg_name: Some("arm"),
        rustc_name: Some("arm"),
    };

    /// # Rustc targets
    /// 
    /// * `armebv7r-none-eabi`
    /// * `armebv7r-none-eabihf`
    /// 
    /// # Notes
    /// 
    /// [Target triple is a misnomer][target triple].
    /// 
    /// `v7r` above is a `<sub>` [target triple] component.
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^armeb"` to obtain the same or similar result.
    /// 
    /// [target triple]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
    pub(super) const ARMEB: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("armeb"),
        vcpkg_name: None,
        rustc_name: Some("armeb"),
    };

    /// # Vcpkg targets
    /// 
    /// * `arm64-windows`
    /// * `arm64-android`
    /// * `arm64-ios`
    /// * `arm64-linux`
    /// * `arm64-mingw-dynamic`
    /// * `arm64-mingw-static`
    /// * `arm64-osx-dynamic`
    /// * `arm64-osx`
    /// * `arm64-uwp`
    /// * `arm64-windows-static-md`
    /// * `arm64-windows-static`
    /// 
    /// # Rustc targets
    /// 
    /// * `aarch64-apple-darwin`
    /// * `aarch64-apple-ios`
    /// * `aarch64-apple-ios-macabi`
    /// * `aarch64-apple-ios-sim`
    /// * `aarch64-apple-tvos`
    /// * `aarch64-apple-watchos-sim`
    /// * `aarch64-fuchsia`
    /// * `aarch64-kmc-solid_asp3`
    /// * `aarch64-linux-android`
    /// * `aarch64-nintendo-switch-freestanding`
    /// * `aarch64-pc-windows-gnullvm`
    /// * `aarch64-pc-windows-msvc`
    /// * `aarch64-unknown-freebsd`
    /// * `aarch64-unknown-hermit`
    /// * `aarch64-unknown-linux-gnu`
    /// * `aarch64-unknown-linux-gnu_ilp32`
    /// * `aarch64-unknown-linux-musl`
    /// * `aarch64-unknown-netbsd`
    /// * `aarch64-unknown-none`
    /// * `aarch64-unknown-none-softfloat`
    /// * `aarch64-unknown-openbsd`
    /// * `aarch64-unknown-redox`
    /// * `aarch64-unknown-uefi`
    /// * `aarch64-uwp-windows-msvc`
    /// * `aarch64-wrs-vxworks`
    /// 
    /// # Reproducibility
    /// 
    /// You can use
    /// 
    /// 1. `vcpkg help triplets | rg "^\s+arm64-"`
    /// 2. `rustc --print target-list | rg "^aarch64-"`
    /// 
    /// to obtain the same or similar results.
    pub(super) const AARCH64_AKA_ARM64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("aarch64"),
        vcpkg_name: Some("arm64"),
        rustc_name: Some("aarch64"),
    };

    /// # Rustc targets
    /// 
    /// * `aarch64_be-unknown-linux-gnu`
    /// * `aarch64_be-unknown-linux-gnu_ilp32`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^aarch64_be"` to obtain the same or similar result.
    pub (super) const AARCH64_BE: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("aarch64_be"),
        vcpkg_name: None,
        rustc_name: Some("aarch64_be"),
    };

    /// # Rustc targets
    /// 
    /// * `arm64_32-apple-watchos`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^arm64_32"` to obtain the same or similar result.
    pub(super) const AARCH64_32_AKA_ARM64_32: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("aarch64_32"),
        vcpkg_name: None,
        rustc_name: Some("arm64_32"),
    };

    /// # Rustc targets
    /// 
    /// * `avr-unknown-gnu-atmega328`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^avr"` to obtain the same or similar result.
    pub(super) const AVR: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("avr"),
        vcpkg_name: None,
        rustc_name: Some("avr"),
    };

    /// # Rustc targets
    /// 
    /// * `bpfel-unknown-none`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^bpfel"` to obtain the same or similar result.
    pub(super) const BPFEL: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("bpfel"),
        vcpkg_name: None,
        rustc_name: Some("bpfel"),
    };

    /// # Rustc targets
    /// 
    /// * `bpfeb-unknown-none`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^bpfeb"` to obtain the same or similar result.
    pub(super) const BPFEB: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("bpfeb"),
        vcpkg_name: None,
        rustc_name: Some("bpfeb"),
    };

    /// # Rustc targets
    /// 
    /// * `hexagon-unknown-linux-musl`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^hexagon"` to obtain the same or similar result.
    pub(super) const HEXAGON: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("hexagon"),
        vcpkg_name: None,
        rustc_name: Some("hexagon"),
    };

    /// # Rustc targets
    /// 
    /// * `mips-unknown-linux-gnu`
    /// * `mips-unknown-linux-musl`
    /// * `mips-unknown-linux-uclibc`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^mips-"` to obtain the same or similar result.
    pub(super) const MIPS: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("mips"),
        vcpkg_name: None,
        rustc_name: Some("mips"),
    };

    /// # Rustc targets
    /// 
    /// * `mipsel-sony-psp`
    /// * `mipsel-unknown-linux-gnu`
    /// * `mipsel-unknown-linux-musl`
    /// * `mipsel-unknown-linux-uclibc`
    /// * `mipsel-unknown-none`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^mipsel"` to obtain the same or similar result.
    pub(super) const MIPSEL: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("mipsel"),
        vcpkg_name: None,
        rustc_name: Some("mipsel"),
    };

    /// # Rustc targets
    /// 
    /// * `mips64-openwrt-linux-musl`
    /// * `mips64-unknown-linux-gnuabi64`
    /// * `mips64-unknown-linux-muslabi64`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^mips64-"` to obtain the same or similar result.
    pub(super) const MIPS64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("mips64"),
        vcpkg_name: None,
        rustc_name: Some("mips64"),
    };

    /// # Rustc targets
    /// 
    /// * `mips64el-unknown-linux-gnuabi64`
    /// * `mips64el-unknown-linux-muslabi64`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^mips64el"` to obtain the same or similar result.
    pub(super) const MIPS64EL: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("mips64el"),
        vcpkg_name: None,
        rustc_name: Some("mips64el"),
    };

    /// # Rustc targets
    /// 
    /// * `msp430-none-elf`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^msp430"` to obtain the same or similar result.
    pub(super) const MSP430: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("msp430"),
        vcpkg_name: None,
        rustc_name: Some("msp430"),
    };

    /// # Rustc targets
    /// 
    /// * `powerpc-unknown-freebsd`
    /// * `powerpc-unknown-linux-gnu`
    /// * `powerpc-unknown-linux-gnuspe`
    /// * `powerpc-unknown-linux-musl`
    /// * `powerpc-unknown-netbsd`
    /// * `powerpc-unknown-openbsd`
    /// * `powerpc-wrs-vxworks`
    /// * `powerpc-wrs-vxworks-spe`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^powerpc-"` to obtain the same or similar result.
    pub(super) const PPC_AKA_POWERPC: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("ppc"),
        vcpkg_name: None,
        rustc_name: Some("powerpc"),
    };

    /// # Rustc targets
    /// 
    /// * `powerpc64-unknown-freebsd`
    /// * `powerpc64-unknown-linux-gnu`
    /// * `powerpc64-unknown-linux-musl`
    /// * `powerpc64-wrs-vxworks`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^powerpc64-"` to obtain the same or similar result.
    pub(super) const PPC64_AKA_POWERPC64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("ppc64"),
        vcpkg_name: None,
        rustc_name: Some("powerpc64"),
    };

    /// # Vcpkg targets
    /// 
    /// * `ppc64le-linux`
    /// 
    /// # Rustc targets
    /// 
    /// * `powerpc64le-unknown-freebsd`
    /// * `powerpc64le-unknown-linux-gnu`
    /// * `powerpc64le-unknown-linux-musl`
    /// 
    /// # Reproducibility
    /// 
    /// You can use
    /// 
    /// 1. `vcpkg help triplets | rg "^\s+ppc64le"`
    /// 2. `rustc --print target-list | rg "^powerpc64le-"`
    /// 
    /// to obtain the same or similar results.
    pub(super) const PPC64LE_AKA_POWERPC64LE: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("ppc64le"),
        vcpkg_name: Some("ppc64le"),
        rustc_name: Some("powerpc64le"),
    };

    /// # Rustc targets
    ///
    /// * `riscv32gc-unknown-linux-gnu`
    /// * `riscv32gc-unknown-linux-musl`
    /// * `riscv32i-unknown-none-elf`
    /// * `riscv32im-unknown-none-elf`
    /// * `riscv32imac-unknown-none-elf`
    /// * `riscv32imac-unknown-xous-elf`
    /// * `riscv32imc-esp-espidf`
    /// * `riscv32imc-unknown-none-elf`
    /// 
    /// # Notes
    /// 
    /// [Target triple is a misnomer][target triple].
    /// 
    /// `gc`, `i`, `im`, `imac`, and `imc` above are `<sub>` [target triple] component. Learn more about RISC-V
    /// targets [here](https://internals.rust-lang.org/t/why-rustcs-risc-v-targets-attach-combinations-of-isa-to-instruction-base-sets/13748).
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^riscv32"` to obtain the same or similar result.
    /// 
    /// [target triple]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
    pub(super) const RISCV32: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("riscv32"),
        vcpkg_name: None,
        rustc_name: Some("riscv32"),
    };

    /// # Rustc targets
    ///
    /// * `riscv64gc-unknown-freebsd`
    /// * `riscv64gc-unknown-linux-gnu`
    /// * `riscv64gc-unknown-linux-musl`
    /// * `riscv64gc-unknown-none-elf`
    /// * `riscv64imac-unknown-none-elf`
    /// 
    /// # Notes
    /// 
    /// [Target triple is a misnomer][target triple].
    /// 
    /// `gc` and `imac` above are `<sub>` [target triple] component. Learn more about RISC-V
    /// targets [here](https://internals.rust-lang.org/t/why-rustcs-risc-v-targets-attach-combinations-of-isa-to-instruction-base-sets/13748).
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^riscv64"` to obtain the same or similar result.
    /// 
    /// [target triple]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
    pub(super) const RISCV64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("riscv64"),
        vcpkg_name: None,
        rustc_name: Some("riscv64"),
    };

    /// # Rustc targets
    /// 
    /// * `sparc-unknown-linux-gnu`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^sparc-"` to obtain the same or similar result.
    pub(super) const SPARC: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("sparc"),
        vcpkg_name: None,
        rustc_name: Some("sparc"),
    };

    /// # Rustc targets
    /// 
    /// * `sparcv9-sun-solaris`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^sparcv9-"` to obtain the same or similar result.
    pub(super) const SPARCV9: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("sparcv9"),
        vcpkg_name: None,
        rustc_name: Some("sparcv9"),
    };

    /// # Warning
    /// 
    /// The writer (@JohnScience) has no idea what this architecture is.
    /// There is a [HAL SPARC64 entry in Wikipedia] but
    /// [LLVM does not consider it a separate architecture](https://en.wikipedia.org/wiki/HAL_SPARC64).
    /// 
    /// # Rustc targets
    /// 
    /// * `sparc64-unknown-linux-gnu`
    /// * `sparc64-unknown-netbsd`
    /// * `sparc64-unknown-openbsd`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^sparc64-"` to obtain the same or similar result.
    /// 
    /// [HAL SPARC64 entry in Wikipedia]: https://en.wikipedia.org/wiki/HAL_SPARC64
    pub(super) const SPARC64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: None,
        vcpkg_name: None,
        rustc_name: Some("sparc64"),
    };

    /// # Vcpkg targets
    /// 
    /// * `s390x-linux`
    /// 
    /// # Rustc targets
    /// 
    /// * `s390x-unknown-linux-gnu`
    /// * `s390x-unknown-linux-musl`
    /// 
    /// # Reproducibility
    /// 
    /// You can use 
    /// 
    /// 1. `vcpkg help triplets | rg "^\s+s390x"`
    /// 2. `rustc --print target-list | rg "^s390x"`
    /// 
    /// to obtain the same or similar result.
    pub(super) const SYSTEMZ_AKA_S390X: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("systemz"),
        vcpkg_name: Some("s390x"),
        rustc_name: Some("s390x"),
    };

    /// # Rustc targets
    ///
    /// * `thumbv4t-none-eabi`
    /// * `thumbv6m-none-eabi`
    /// * `thumbv7a-pc-windows-msvc`
    /// * `thumbv7a-uwp-windows-msvc`
    /// * `thumbv7em-none-eabi`
    /// * `thumbv7em-none-eabihf`
    /// * `thumbv7m-none-eabi`
    /// * `thumbv7neon-linux-androideabi`
    /// * `thumbv7neon-unknown-linux-gnueabihf`
    /// * `thumbv7neon-unknown-linux-musleabihf`
    /// * `thumbv8m.base-none-eabi`
    /// * `thumbv8m.main-none-eabi`
    /// * `thumbv8m.main-none-eabihf`
    /// 
    /// # Notes
    /// 
    /// [Target triple is a misnomer][target triple].
    /// 
    /// `v4t`, `v6m`, `v7a`, `v7em`, `v7m`, `v7neon`, `v8m` above are `<sub>` [target triple] components.
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^thumb"` to obtain the same or similar result.
    /// 
    /// [target triple]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
    pub(super) const THUMB: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("thumb"),
        vcpkg_name: None,
        rustc_name: Some("thumb"),
    };

    /// # Vcpkg targets
    /// 
    /// * `x86-windows`
    /// * `x86-android`
    /// * `x86-freebsd`
    /// * `x86-ios`
    /// * `x86-mingw-dynamic`
    /// * `x86-mingw-static`
    /// * `x86-uwp`
    /// * `x86-windows-static-md`
    /// * `x86-windows-static`
    /// * `x86-windows-v120`
    /// 
    /// You can use `vcpkg help triplets | rg "^\s+x86"` to obtain the same or similar result.
    pub(super) const X86: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("x86"),
        vcpkg_name: Some("x86"),
        rustc_name: None,
    };

    /// # Vcpkg targets
    /// 
    /// * `x64-linux`
    /// * `x64-osx`
    /// * `x64-uwp`
    /// * `x64-windows-static`
    /// * `x64-windows`
    /// * `x64-android`
    /// * `x64-freebsd`
    /// * `x64-ios`
    /// * `x64-linux-dynamic`
    /// * `x64-linux-release`
    /// * `x64-mingw-dynamic`
    /// * `x64-mingw-static`
    /// * `x64-openbsd`
    /// * `x64-osx-dynamic`
    /// * `x64-osx-release`
    /// * `x64-windows-release`
    /// * `x64-windows-static-md`
    /// 
    /// # Rustc targets
    /// 
    /// * `x86_64-apple-darwin`
    /// * `x86_64-apple-ios`
    /// * `x86_64-apple-ios-macabi`
    /// * `x86_64-apple-tvos`
    /// * `x86_64-apple-watchos-sim`
    /// * `x86_64-fortanix-unknown-sgx`
    /// * `x86_64-fuchsia`
    /// * `x86_64-linux-android`
    /// * `x86_64-pc-solaris`
    /// * `x86_64-pc-windows-gnu`
    /// * `x86_64-pc-windows-gnullvm`
    /// * `x86_64-pc-windows-msvc`
    /// * `x86_64-sun-solaris`
    /// * `x86_64-unknown-dragonfly`
    /// * `x86_64-unknown-freebsd`
    /// * `x86_64-unknown-haiku`
    /// * `x86_64-unknown-hermit`
    /// * `x86_64-unknown-illumos`
    /// * `x86_64-unknown-l4re-uclibc`
    /// * `x86_64-unknown-linux-gnu`
    /// * `x86_64-unknown-linux-gnux32`
    /// * `x86_64-unknown-linux-musl`
    /// * `x86_64-unknown-netbsd`
    /// * `x86_64-unknown-none`
    /// * `x86_64-unknown-none-linuxkernel`
    /// * `x86_64-unknown-openbsd`
    /// * `x86_64-unknown-redox`
    /// * `x86_64-unknown-uefi`
    /// * `x86_64-uwp-windows-gnu`
    /// * `x86_64-uwp-windows-msvc`
    /// * `x86_64-wrs-vxworks`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^x86_64"` to obtain the same or similar result.
    pub(super) const X86_64_AKA_X64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("x86_64"),
        vcpkg_name: Some("x64"),
        rustc_name: Some("x86_64"),
    };

    /// # Rustc targets
    /// 
    /// * `nvptx64-nvidia-cuda`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^nvptx"` to obtain the same or similar result.
    pub(super) const NVPTX64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("nvptx64"),
        vcpkg_name: None,
        rustc_name: Some("nvptx64"),
    };

    /// # Vcpkg targets
    /// 
    /// * `wasm32-emscripten`
    /// 
    /// # Rustc targets
    /// 
    /// * `wasm32-unknown-emscripten`
    /// * `wasm32-unknown-unknown`
    /// * `wasm32-wasi`
    /// 
    /// # Reproducibility
    /// 
    /// You can use 
    /// 
    /// 1. `vcpkg help triplets | rg "^\s+wasm32"`
    /// 2. `rustc --print target-list | rg "^wasm32"`
    /// 
    /// to obtain the same or similar result.
    pub(super) const WASM32: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("wasm32"),
        vcpkg_name: Some("wasm32"),
        rustc_name: Some("wasm32"),
    };

    /// # Rustc targets
    /// 
    /// * `wasm64-unknown-unknown`
    /// 
    /// # Reproducibility
    /// 
    /// You can use `rustc --print target-list | rg "^wasm64"` to obtain the same or similar result.
    pub(super) const WASM64: Self = Self {
        #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
        llvm_name: Some("wasm64"),
        vcpkg_name: None,
        rustc_name: Some("wasm64"),
    };

    #[cfg(any(test,feature = "semver_exempt_llvm_ttc"))]
    decl_supported_only_by_llvm!();
}