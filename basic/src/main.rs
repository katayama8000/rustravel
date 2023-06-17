fn add_one_v1(x: i32) -> i32 {
    x + 1
}

fn main() {
    // クロージャー
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one_v2(1));
    println!("{}", add_one_v1(1));

    let x: i32 = 4;
    let equal_to_x = |z| z == x;
    let y: i32 = 4;
    println!("{}", equal_to_x(y));

    // 文字列に!!!を追加するクロージャー
    let add_exclamation = |s: String| -> String { format!("{}!!!", s) };
    let s = String::from("hello");
    println!("{}", add_exclamation(s));

    // イテレーター
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    let v: Vec<i32> = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let add_world_to_vec = |v: &mut Vec<String>| {
        v.push(String::from("world"));
    };
    let mut vec: Vec<String> = vec![String::from("hello")];
    add_world_to_vec(&mut vec);
    println!("{:?}", vec);
    let add_world_to_string = |s: &mut String| {
        s.push_str("world");
    };
    let mut s: String = String::from("hello");
    add_world_to_string(&mut s);
    println!("{}", s);
}
