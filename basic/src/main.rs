mod calc;
mod lib01;
mod lib02;
use lib01::add::add_numbers;
use lib02::multiply::{divide_numbers, multiply_numbers};
use rand::Rng;

fn main() {
    println!("1 + 2 = {}", add_numbers(1, 2));
    let mut rng = rand::thread_rng();
    let n1: i32 = rng.gen();
    let n2: i32 = rng.gen();
    println!("{} + {} = {}", n1, n2, add_numbers(n1, n2));
    println!("{} + {} = {}", n1, n2, calc::add(n1, n2));
    let num1: i32 = rng.gen();
    let num2: i32 = rng.gen();
    println!("{} * {} = {}", 3, 2, multiply_numbers(3, 2));
    println!("{} * {} = {}", 6, 2, divide_numbers(6, 2));
}

// calc モジュールを定義する
// mod calc {
//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }
// }
