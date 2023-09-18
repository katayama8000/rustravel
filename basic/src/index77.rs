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
        money:u32
    };

    let mut sum = 0;
    let mut day = 0;
    while sum < money {
        day += 1;
        sum += day;
    }

    println!("{}", day);
}
