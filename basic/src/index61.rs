use std::cmp::Reverse;

// カスタム構造体を定義
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // サンプルデータを作成
    let mut people: Vec<Person> = vec![
        Person {
            name: String::from("Alice"),
            age: 25,
        },
        Person {
            name: String::from("Bob"),
            age: 30,
        },
        Person {
            name: String::from("Charlie"),
            age: 20,
        },
    ];

    // 年齢を基準にソート
    people.sort_by_key(|person| person.age);

    // ソートされた結果を表示
    for person in &people {
        println!("{:?}", person);
    }

    people.sort_by_key(|person| Reverse(person.age));
    for person in &people {
        println!("{:?} - {:?}", person.age, person.name);
    }
}
