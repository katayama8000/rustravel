fn main() {
    // 参照と借用
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // 可変参照の制約
    // 可変参照は、特定のスコープで特定のデータに対して一つしか存在できない
    // 不変な参照をしている間は、可変な参照をすることはできない
    let mut y = String::from("hello");
    let r1 = &mut y;
    // let r2 = &mut s; // NG
    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
