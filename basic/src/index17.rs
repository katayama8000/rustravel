fn main() {
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let mut s1 = String::from("hello");
    let s2 = " world";
    s1.push_str(s2);
    println!("{}", s1);
    println!("{}", s2);

    let mut s3: String = String::new();
    println!("{}", s3);

    s3.push_str("hello");
    println!("{}", s3);

    let s4: char = 'a';
    let s5: String = s4.to_string();
    let s6: &str = "aaa";

    let s7 = String::from("hello");
    let answer = &s7[0..1];
}
