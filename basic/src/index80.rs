use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let count = (a..=b).count();
    println!("{}", count);
}
