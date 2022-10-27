/// Read about `rustc`'s platfrom support [here](https://doc.rust-lang.org/nightly/rustc/platform-support.html#platform-support)
pub(crate) enum RustcSupportTier {
    Unsupported,
    /// "Guaranteed to work"
    One,
    /// "Guaranteed to build"
    Two,
    /// The platfrom that Rust codebase has support for
    Three,
}