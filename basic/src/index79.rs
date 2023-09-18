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
        c:u32,
        d:u32,
    };

    if b > c {
        println!("{}", -1);
        return;
    }

    let mut blue = a;
    let mut red = 0;

    for i in 1.. {
        blue += b;
        red += c;

        if red * d >= blue {
            println!("{}", i);
            break;
        }
    }
}
