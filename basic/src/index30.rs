fn add_one_v1(x: i32) -> i32 {
    x + 1
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size: Vec<Shoe> = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 関連型
    type Item = u32;
    // nextメソッド
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            // Someは値を持つ
            Some(self.count)
        } else {
            // Noneは値を持たない
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(x, y)| x * y)
        .filter(|x: &u32| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
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
    let doubled: Vec<i32> = v.iter().map(|x: &i32| x * 2).collect();
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

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x: &i32| x + 1).collect();
    println!("{:?}", v2);
}
