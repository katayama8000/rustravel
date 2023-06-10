fn main() {
    // 所有権
    let s1: String = String::from("hello");
    println!("{}", s1);
    let s2: String = s1;
    // println!("{}", s1); // error
    let s3: String = s2.clone();
    println!("{}", s3);

    print_str_fn(s3);
    // println!("{}", s3); // error

    let x1: i32 = 5;
    print_num_fn(x1);
    println!("{}", x1);

    let s4: String = String::from("hello");
    let s5: String = move_back_fn(s4);
    println!("{}", s5);

    let s6: String = String::from("hello");
    borrow_str_fn(&s6);
    println!("{}", s6);

    let mut s7: String = String::from("hello");
    borrow_mut_str_fn(&mut s7);
    println!("{}", s7);

    // 参照の規則
    // 任意のタイミングで１つの可変参照か不変な参照幾つでものどちらかを行うことができる
    // 参照は常に有効でなければならない
}

fn print_str_fn(s: String) {
    println!("{}", s);
}

fn print_num_fn(x: i32) {
    println!("{}", x);
}

fn move_back_fn(s: String) -> String {
    s
}

fn borrow_str_fn(s: &String) {
    println!("{}", s);
}

fn borrow_mut_str_fn(s: &mut String) {
    s.push_str(" world");
    println!("{}", s)
}
