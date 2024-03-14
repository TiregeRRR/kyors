use crate::fileapi::file_res::FileRes;
use std::io::{self, BufRead};

pub trait Explorer {
    fn open(&mut self, directory: String) -> Result<Vec<FileRes>, io::Error>;
    fn copy(&self, file: String) -> Result<(String, Box<dyn BufRead>), io::Error>;
    fn paste(&self, file_name: String, reader: Box<dyn BufRead>) -> Result<(), io::Error>;
    fn delete(&self, file_name: String) -> Result<(), io::Error>;
}
