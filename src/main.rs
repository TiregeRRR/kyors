use fileapi::explorer::Explorer;

use crate::local_files::local_files::LocalFiles;

pub mod fileapi;
pub mod local_files;

fn main() {
    let l = LocalFiles::default();
    let mut r = LocalFiles::default();
    r.open(String::from("src")).unwrap();
    let (file_name, reader) = l.copy(String::from("Cargo.toml")).unwrap();
    r.paste(file_name, reader).unwrap();
    l.delete(String::from("somedir")).unwrap();
}
