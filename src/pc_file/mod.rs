use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod pc_file;
mod pc_files;

pub(crate) use self::pc_file::PcFile;
pub(crate) use self::pc_files::PcFiles;
