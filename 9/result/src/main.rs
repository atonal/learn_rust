use std::fs::File;
use std::io::{self, ErrorKind, Read};
// use std::io::ErrorKind;
// use std::io;
// use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };

    // let f = File::open("bar.txt").unwrap();
    let f = File::open("bar.txt").expect("Failed to open bar.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e), // early exit from function
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// One liner function: fs::read_to_string("hello.txt"))
