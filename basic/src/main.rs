fn main() {
    // play with vectors
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v[0]);
    v.push(6);
    println!("v: {:?}", v);
    // 全て２倍
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("v2: {:?}", v2);
    // 全て３倍
    let v3 = v.iter().map(|x| x * 3);
    println!("v3: {:?}", v3);
    let iter: std::slice::Iter<'_, i32> = v.iter();
    println!("iter: {:?}", iter);
    let v4: Option<&i32> = v.iter().next();
    println!("v4: {:?}", v4);
    // someの中身を取り出す
    let v5: i32 = v.iter().next().unwrap().clone();
    let v6: i32 = v.iter().next().unwrap().clone();
    println!("v5: {:?}", v5);
    println!("v6: {:?}", v6);
    // 偶数だけ取り出す
    let v7: Vec<i32> = v.iter().filter(|&x| *x % 2 == 0).map(|x| x * 1).collect();
    let v8: Vec<i32> = v.iter().filter(|&x| *x % 2 == 0).cloned().collect();
    let v9: Vec<i32> = v.iter().filter(|&x| *x % 2 == 0).cloned().collect();
    println!("v7: {:?}", v7);
    println!("v8: {:?}", v8);
    // v7とv8を比較
    println!("v7 == v8: {:?}", v7 == v8);
    // v7とv9を比較
    println!("v7 == v9: {:?}", v7 == v9);
    // let str0: &str = "hello";
    let str: String = String::from("hello");
    let str_ref: &str = &String::from("hello");
    if str == str_ref {
        println!("equal");
    } else {
        println!("not equal");
    }

    let mut foo = Foo { x: 13 };
    let f = &mut foo;

    f.x = 13;
    // ここで変数fはドロップします。=> つまり、可変な借用もここでドロップします。

    println!("{}", foo.x);

    // 可変な借用はドロップされているため変更可能となります。
    foo.x = 7;

    // 変数fooの所有権を関数do_somethingに移動します。
    do_something(foo);
    print!("------------------\n");

    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // 所有者の値を取得
    *f = 13; // 参照の所有者の値を設定
    println!("{}", bar);
    println!("{}", foo);
}

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // ここで引数fはドロップします。
}
