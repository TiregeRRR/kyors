use std::path::{Path, PathBuf};

use crate::fileapi::explorer;
use crate::fileapi::file_res::FileRes;

pub struct LocalFiles {
    cur_path: PathBuf,
}

impl explorer::Explorer for LocalFiles {
    fn open(&mut self, directory: String) -> Result<Vec<FileRes>, std::io::Error> {
        match directory.as_str() {
            ".." => if let Some(path) = self.cur_path.parent() {},
            _ =>
        }
    }
}

impl Default for LocalFiles {
    fn default() -> Self {
        LocalFiles {
            //TODO not unwrap
            cur_path: std::env::current_dir().unwrap(),
        }
    }
}
