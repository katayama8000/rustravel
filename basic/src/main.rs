
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
        s: String,
    }

    let mut ret = String::new();

    for c in s.chars().rev() {
        let rev = match c {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => unreachable!(),
        };

        ret.push(rev)
    }
    println!("{}", ret);
}
