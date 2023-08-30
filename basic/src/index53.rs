#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let r;

    let x = 5;
    r = &x;

    println!("{}", r);

    let str1 = String::from("abcd");
    let str2 = String::from("efg");

    let ret = longest(str1.as_str(), str2.as_str());
    println!("{}", ret);

    let ret2 = fnc("hoge");
    println!("{}", ret2);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); // '.'が見つかりませんでした
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt is {:?}", i.part)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn fnc<'a>(x: &str) -> &'a str {
    let y = "str";
    y
}
