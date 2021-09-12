enum Result<T, E> {
    OK(T),
    Err(E),
}

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!(
                    "Tried to create file but there was a ploblem:{:?}", 
                    e)
            }
        },
        Err(error) =>{
            panic!(
                "There was a problem opening the file:{:?}",
                error
                )
        },
    };
    //let v = vec![1, 2, 3];
    //    v[99];
    //    panic!("crash and burn")
}
