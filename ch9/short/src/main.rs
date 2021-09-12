use std::{fs::File, io::Read};
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
//    let f = File::open("hello.txt");
//
//    let mut f = match f {
//        Ok(file) => file,
//        Err(e) => return Err(e),
//    };
//
//    let mut s = String::new();
//
//    match f.read_to_string(&mut s) {
//        Ok(_) => Ok(s),
//        Err(e) => Err(e),
//    }
}
fn main() {
    read_username_from_file();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt").unwrap();
}
