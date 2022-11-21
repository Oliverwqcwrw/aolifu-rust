use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error);
        }
    };
}

pub fn open_file_two() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error create file: {:?}", error)
            },
            other_error => panic!("Error open the file: {:?}", other_error)

        }
    };
}

pub fn open_file_three() {
    let f = File::open("hello.txt").unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(
                    |error| {
                        panic!("error create file {:?}", error);
                    }
                )
            } else {
                panic!("error opening the file {:?}", error);
            }
        }
    );
}

pub fn spread_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    // the last expression is result
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn spread_error_two() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn spread_error_three() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}