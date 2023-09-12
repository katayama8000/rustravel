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
        x: i32,
        y: i32,
    }

    if x != y {
        println!("{}", 3 - (x + y));
    } else {
        println!("{}", x);
    }
}
