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
        a: usize,
        b: usize,
        c: usize,
    }

    // let a = 1;
    // let b = 1;
    // let c = 1;

    if a == b && b == c && c == a {
        println!("{}", a);
        return;
    }

    if a != b && b != c && c != a {
        println!("{}", 0);
        return;
    }

    if a == b {
        println!("{}", c)
    } else if b == c {
        println!("{}", a)
    } else {
        println!("{}", b)
    }
}
