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
    // input! {
    //     a: usize,
    //     b: usize,
    //     c: usize,
    // }

    let a = 2;
    let b = 3;
    let c = 3;

    if a == b && b == c && c == a {
        println!("{}", a)
    }

    if a != b && b != c && c != a {
        println!("{}", 0)
    }

    if a == b {
        println!("{}", c)
    } else if b == c {
        println!("{}", a)
    } else {
        println!("{}", b)
    }
}
