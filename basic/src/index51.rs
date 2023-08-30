#[derive(Copy, Clone)]
struct C;

struct Z;
fn main() {
    // copyしているので、所有権のエラーは出ない
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;

    // #[derive(Copy, Clone)]がないと、所有権がmoveしてコンパイルエラーになる
    let z0 = Z;
    let _z1 = z0;
    // let _z2 = z0;

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");

    // let result = longest(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
