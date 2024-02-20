use fileapi::explorer::Explorer;

use crate::local_files::local_files::LocalFiles;

pub mod fileapi;
pub mod local_files;

fn main() {
    let mut l = LocalFiles::default();
    print_dir(&mut l);
    print_dir(&mut l);
}

fn print_dir(d: &mut dyn Explorer) {
    println!("{:?}", d.open(String::from("..")));
}
