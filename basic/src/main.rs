fn main() {
    // 文字列
    let s1: String = String::from("Hello World!!"); // これはString
    let s2: &str = &s1[1..4]; // Stringのスライスは&str
    let s3: &str = "ABC 123"; // 文字列リテラルは&str
    let s4: &str = &s3[0..2]; // 文字列リテラルのスライスも&str
    println!("{}{}{}{}", s1, s2, s3, s4);
    let s5: &'static str = "DEF";
    println!("{}", s5);
    let mut s1: String = String::from("ABC");
    let s2: String = String::from("DEF");
    s1 = s1 + "!?" + &s2;
}
