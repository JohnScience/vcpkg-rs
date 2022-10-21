#[derive(Clone, Debug)]
pub(crate) struct Port {
    // dlls if any
    pub(crate) dlls: Vec<String>,

    // libs (static or import)
    pub(crate) libs: Vec<String>,

    // ports that this port depends on
    pub(crate) deps: Vec<String>,
}
