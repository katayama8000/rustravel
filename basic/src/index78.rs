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
        a:u32,
        b:u32,
        c:u32
    };

    let mut arr = [a, b, c];
    arr.sort();
    let ans = arr.iter().skip(1).take(2).sum();
    println!("{}", ans);
}
