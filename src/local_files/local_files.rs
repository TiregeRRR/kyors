use std::fs;
use std::path::PathBuf;

use crate::fileapi::explorer;
use crate::fileapi::file_res::FileRes;

pub struct LocalFiles {
    cur_path: PathBuf,
}

impl LocalFiles {
    fn get_current_files(&mut self) -> Result<Vec<FileRes>, std::io::Error> {
        let res_files = fs::read_dir(self.cur_path.as_path())?;

        let mut res: Vec<FileRes> = Vec::new();

        res.extend(res_files.map(|file| FileRes {
            name: file.unwrap().file_name().into_string().unwrap(),
        }));

        Ok(res)
    }
}

impl explorer::Explorer for LocalFiles {
    fn open(&mut self, directory: String) -> Result<Vec<FileRes>, std::io::Error> {
        match directory.as_str() {
            ".." => {
                self.cur_path.pop();
                let files = self.get_current_files()?;

                Ok(files)
            }
            _ => {
                self.cur_path.push(directory);

                let files = self.get_current_files()?;

                Ok(files)
            }
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
