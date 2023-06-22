use std::fs::File;
mod commands;
mod lib01;
use lib01::add::add_numbers;

// trait
trait Tweet {
    fn tweet(&self);
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }
    fn shout(&self) {
        println!("Uooooooooooooooo");
    }
}

struct Dove;
struct Duck;

fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};
    duck.tweet();
    duck.tweet_twice();
    duck.shout();

    let tuple1: (i32, i32) = make_tuple(1, 2);
    let tuple2: (&str, &str) = make_tuple("Hello", "World");
    let tuple3: (f32, f32) = make_tuple(1.0, 2.0);
    println!("tuple1: {:?}", tuple1);
    println!("tuple2: {:?}", tuple2);
    println!("tuple3: {:?}", tuple3);

    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => {
            println!("file: {:?}", file);
        }
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    commands::command_a();
    let num = lib01::add::add_numbers(1, 2);
    println!("num: {}", num);
    let num = add_numbers(1, 5);
    println!("num: {}", num);
    let num = lib01::sub::sub_numbers(1, 2);
    println!("num: {}", num);
}
