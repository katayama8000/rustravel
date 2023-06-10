use std::{fs::File, io::ErrorKind};

fn main() {
    // error
    // panic!("crash and burn")
    let f1 = File::open("hello.txt");
    let _f2 = match f1 {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                }
            } else {
                panic!("There was a problem opening the file: {:?}", error)
            }
        }
    };

    let f: Result<File, std::io::Error> = File::open("hello.txt");
    let _f: File = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}
