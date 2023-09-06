#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    // input! {
    //     n: usize,
    //     _mountains: [(String, i32); n],
    // };

    // let mut cloned_mountains = mountains.clone();
    let mut cloned_mountains: Vec<(String, i32)> = vec![
        ("Mount Everest".to_string(), 8848),
        ("K2".to_string(), 8611),
        ("Kangchenjunga".to_string(), 8586),
        ("Lhotse".to_string(), 8516),
        ("Makalu".to_string(), 8485),
    ];
    cloned_mountains.sort_by_key(|&(_, height)| Reverse(height));

    let tpl: (i32, String) = (30, "aaa".to_string());
    println!("{}", tpl.0);

    let mut people: Vec<Person> = vec![
        Person {
            name: "mike".to_string(),
            age: 32,
        },
        Person {
            name: "john".to_string(),
            age: 30,
        },
    ];

    let second_highest_mountain = &cloned_mountains[1].0;
    people.sort_by_key(|person| Reverse(person.age));
    println!("{:?}", people);
    let second = people[1].age;
    println!("{}", second);

    println!("{}", second_highest_mountain);
}
