fn main() {
    // rust を学んでいく
    // 1. 変数
    // immutable
    let a = 10;
    // reference
    let aref1 = &a;
    // 代入
    // a = 20; // error[E0384]: cannot assign twice to immutable variable `a`
    println!("{}, {}", a, aref1); // borrow check!! - OK
    // mutable
    let mut b = 10;
    // 代入
    b = 20;
    println!("{}", b); // borrow check!! - OK
    // 2. 関数
    // 足し算
    println!("{}", add(10, 20));
    let c = add(10, 20);
    println!("{}", c);
    // 3. 型
    // 3.1 基本型
    // 文字列
    let s = "hello";
    println!("{}", s);
    // 文字列の連結
    let s1 = "hello";
    let s2 = "world";
    let s3 = s1.to_string() + s2;
    println!("{}", s3);
    // 3.2 複合型
    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    // 配列
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    // 4. 制御構文
    // 4.1 if
    let a = 10;
    if a == 10 {
        println!("a is 10");
    } else {
        println!("a is not 10");
    }
    // 4.2 loop
    let mut a = 0;
    loop {
        a += 1;
        println!("{}", a);
        if a == 10 {
            break;
        }
    }
    // 4.3 while
    let mut a = 0;
    while a < 10 {
        a += 1;
        println!("{}", a);
    }
    // 4.4 for
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("{}", i);
    }

    // for while loop の違い
    // for loop は、配列の要素を順番に取り出す
    // while loop は、条件を満たす間、繰り返す

    // 5. 所有権
    // ex
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error[E0382]: borrow of moved value: `s1`
    println!("{}, world!", s2);
    


    



}

// 2. 関数
// 足し算
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// 引き算
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
// 掛け算
fn mul(a: i32, b: i32) -> i32 {
    a * b
}
// 割り算
fn div(a: i32, b: i32) -> i32 {
    a / b
}
// i32とは
