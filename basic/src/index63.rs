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
          dices: [usize; 3]
    };
    let dices: [i64; 3] = [a1, a2, a3];
    let sum: i64 = dices.iter().map(|&num| reverse_dice(num)).sum();
    println!("{}", sum);
    // for dice in dices {
    //     sum = sum + reverse_dise(dice);
    // }

    // let v: [String; 3] = ["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];

    // for el in v {
    //     println!("{}", el);
    // }
    // let nums: [i64; 3] = [1, 2, 3];

    // for num in nums {
    //     sum = sum + reverse_dise(num);
    // }
}

fn reverse_dice(num: i64) -> i64 {
    7 - num
}
