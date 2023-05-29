use std::str;

fn main() {
    let mut s: String = String::from("hello world");
    let mut s2: String = String::from("hello world");
    let my_string: String = String::from("hello world");
    let mmm: &str = "hello world";
    let mmmString: String = String::from(mmm);

    let word: usize = first_word(&s);
    let word2: &str = first_word_2(&s2);
    let word2: &str = first_word_3(&my_string[..]);

    s.clear(); // error!

    println!("the first word is: {}", word);
    println!("the first word is: {}", word2);
    // sample();

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[2..3];
    println!("{}", a[1]);
    println!("{}", slice[0]);
    println!("------------------");
    let str1: &str = "hello";
    let str2: [char; 5] = ['h', 'e', 'l', 'l', 'o'];
    let str3: String = String::from("hello");
    println!("{}", str1);
    println!("{:?}", str2);
    println!("{}", &str3);
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn sample() -> () {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s: String = String::from("hello");

    let len: usize = s.len();

    let slice1 = &s[0..len];
    let slice2 = &s[..2];
    // 比較
    println!("{}", slice1 == slice2);
}
