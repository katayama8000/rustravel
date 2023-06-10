use std::fs::File;
use std::io::Read;
use std::{io, str};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let f: File = File::open("hello.txt").unwrap();
    println!("{:?}", f);

    let f1: File = File::open("world.txt").expect("Failed to open world.txt");
    println!("{:?}", f1);

    let s: String = read_username_from_file().unwrap();
    println!("{:?}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f: File = File::open("hello.txt")?;
    let mut s: String = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
