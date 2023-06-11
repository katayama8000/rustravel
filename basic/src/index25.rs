fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // lifetime
    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result: &str = largest(string1.as_str(), string2);
    println!("The largest string is {}", result);
}
