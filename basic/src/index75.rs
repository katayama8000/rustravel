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
        mut a: [usize; n]
    };

    a.sort();
    // let mut is_ok = true;
    // for (i, &num) in a.iter().enumerate() {
    //     if i != num {
    //         is_ok = false
    //     }
    // }

    let ans = a
        .iter()
        .enumerate()
        .filter(|&(i, a_i)| i + 1 == *a_i)
        .count()
        == n;

    if ans {
        println!("Yes")
    } else {
        println!("No")
    }
}
