pub mod lib1;
pub mod three;

use three::return_number;

enum Spread {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let some_u8_value: Option<i32> = Some(4);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("nothing")
    }
    lib1::hello();
    let num: usize = return_number::return_number();
    println!("The number is {}", num);

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(6);
    println!("v1 is {:?}", v1);
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(5) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row: Vec<Spread> = vec![
        Spread::Int(3),
        Spread::Float(10.12),
        Spread::Text(String::from("hello")),
    ];

    for i in &row {
        match i {
            Spread::Int(value) => println!("The value is {}", value),
            Spread::Float(value) => println!("The value is {}", value),
            Spread::Text(value) => println!("The value is {}", value),
        }
    }
}
