use std::io::Error;
use std::fmt;

pub trait Write {
    fn write(&mut self, buf :&[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self,fmt: fmt::Arguments) -> Result<(), Error>;
}

fn add_one(x: i32) -> i32{
    x +1
}
fn do_twice(f: fn(i32) -> i32, arg:i32) -> i32{
    f(arg) + f(arg)
}
fn main() {
    let answer = do_twice(add_one , 5);
    println!("{}" ,answer);
}
