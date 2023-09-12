#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n:f32
    };

    match n {
        n if n * 1.08 > 206.0 => println!(":("),
        n if n * 1.08 < 206.0 => println!("Yay!"),
        _ => println!("so-so"),
    }
}
