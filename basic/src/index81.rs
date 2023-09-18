// use proconio::input;
fn main() {
    // input! {
    //     n: i32,
    //     x: i32,
    //     prices: [i32; n],
    // }

    let x = 3;
    let prices = [1, 3];

    let mut total_cost = 0;
    for (i, &price) in prices.iter().enumerate() {
        if i % 2 == 0 {
            total_cost += price;
        } else {
            total_cost += price - 1;
        }
    }

    let ans = prices
        .into_iter()
        .enumerate()
        .map(|(i, a_i)| a_i - if i % 2 == 0 { 0 } else { 1 })
        .sum::<usize>();

    println!("{}", if ans <= x { "Yes" } else { "No" });

    let a = 2;
    let b = 5;
    println!("{}", (a..=b).count())
}
