fn main() {
    // 所有権
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する
    let s1 = String::from("hello");
    let s2 = s1; // s1はこの時点でメモリを解放しているので、s1を参照することはできない
                 // println!("{}, world!", s1); // borrow check!! - NG

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s1 = {}, s2 = {}", s3, s4);

    let x = 1;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // 上はコンパイルエラーにならない
    // これは、i32のような既知のサイズを持つ型はスタックに置かれるため、
    // データのコピーが迅速に行えるためです。
    // これは、ポインタのデータ、長さ、容量がスタックに置かれるString型には当てはまりません。

    // 所有権と関数
    let s = String::from("hello this is String"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない

    // println!("{}", s); // borrow check!! - NG

    let x = 5; // xがスコープに入る

    makes_copy(x);

    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
