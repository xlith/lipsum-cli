use std::fs::File;
use std::io::{self, Read};

pub fn read_from_stdin() -> String {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(len) => if len == 0 {
                break;
            }
            Err(error) => {
                eprintln!("error: {}", error);
                break;
            }
        }
    }
    input
}

pub fn read_from_file(file: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(file).expect("File not found!");
    f.read_to_string(&mut input).expect("Something went wrong reading the file!");
    input
}