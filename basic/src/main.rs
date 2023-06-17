fn main() {
    let s1: String = String::from("hello");
    let s2: &str = "hello";

    let s3: String = s1.clone();

    takes_ownership(s3);
    let length1: usize = calculate_length(&s1);
    println!("{}", length1);
    println!("{}", s1);

    let rets1: &String = return_string2(&s1);
    println!("{}", rets1);
    println!("{}", s1);

    let mut s4: String = String::from("hello");
    change_text(&mut s4);
    println!("{}", s4);

    let s5: &mut String = &mut s4;
    let s6: &mut String = &mut s4;

    let s7: &str = "helloooooo";
    let s8: &str = &s7;
    let s9: &str = &s7;
    println!("{}", s7);
    println!("{}", s8);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn func() -> String {
    let s: String = String::from("hello");
    s
}

// pointerを引数に取る関数
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 文字列を変更する関数
fn return_string1(s: String) -> String {
    s
}

// pointerを引数にとってそれをそのまま返す
fn return_string2(s: &String) -> &String {
    s
}

// 可変参照を引数にとる関数
fn change_text(s: &mut String) {
    s.push_str(", world");
}
