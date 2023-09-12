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
        a: [usize; n],
    }

    let mut sum = 0;
    for i in a {
        if a[i] > 10 {
            sum += a[i] - 10;
        }
    }

    let ans = a
        .into_iter()
        .filter(|a_i| a_i > &10)
        .map(|a_i| a_i - 10)
        .sum::<usize>();

    println!("{}", sum);
}
