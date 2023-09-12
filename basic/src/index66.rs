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
        n: usize,
        k: usize,
    }

    let mut total = 0;

    for i in 1..=n {
        for j in 1..=k {
            let room_number = i * 100 + j;
            total += room_number;
        }
    }

    println!("{}", total);
}
