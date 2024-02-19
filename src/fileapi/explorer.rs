use crate::fileapi::file_res::FileRes;
use std::io;

pub trait Explorer {
    fn open(&mut self, directory: String) -> Result<Vec<FileRes>, io::Error>;
}
