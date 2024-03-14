use std::io::BufReader;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::fileapi::explorer;
use crate::fileapi::file_res::FileRes;

pub struct LocalFiles {
    cur_path: PathBuf,
}

impl LocalFiles {
    fn get_current_files(&mut self) -> Result<Vec<FileRes>, std::io::Error> {
        let res_files = fs::read_dir(self.cur_path.as_path())?;

        let mut res: Vec<FileRes> = Vec::new();

        res.extend(res_files.map(|file| {
            let f = file.unwrap();
            FileRes {
                name: f.file_name().into_string().unwrap(),
                size: f.metadata().unwrap().size(),
            }
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

    fn copy(
        &self,
        file_name: String,
    ) -> Result<(String, Box<dyn std::io::prelude::BufRead>), std::io::Error> {
        let mut copy_path = self.cur_path.clone();
        let path = AsRef::<Path>::as_ref(&file_name);
        copy_path.push(path);
        let file = fs::File::open(copy_path)?;
        let reader = BufReader::new(file);
        Ok((file_name, Box::new(reader)))
    }

    fn paste(
        &self,
        file_name: String,
        mut reader: Box<dyn std::io::prelude::BufRead>,
    ) -> Result<(), std::io::Error> {
        let mut paste_path = self.cur_path.clone();
        paste_path.push(file_name);

        let mut file = std::fs::File::create(paste_path)?;
        io::copy(&mut reader, &mut file)?;

        Ok(())
    }

    fn delete(&self, file_name: String) -> Result<(), io::Error> {
        let mut delete_path = self.cur_path.clone();
        delete_path.push(file_name);

        let file = std::fs::File::open(delete_path.clone())?;

        let file_metadata = file.metadata()?;
        if file_metadata.is_dir() {
            fs::remove_dir_all(delete_path)?;
        } else {
            fs::remove_file(delete_path)?;
        }

        Ok(())
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
