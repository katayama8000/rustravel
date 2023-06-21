fn main() {
    // lifetime
    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result: &str = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    {
        let string3: String = String::from("long string is long");
        {
            let string4: String = String::from("xyz");
            let result: &str = longest(string3.as_str(), string4.as_str());
        }
        println!("The longest string is {}", result);
    }
}

// この関数は、引数xとyのどちらが長いかを比較し、長い方の参照を返す
// しかし、この関数はコンパイルできない
// なぜなら、xとyのどちらが返されるかは、関数の呼び出し元のコードに依存するから
// つまり、関数の呼び出し元のコードによっては、xが返されるかもしれないし、yが返されるかもしれない
// このような場合、関数の呼び出し元のコードによって、関数の返り値のライフタイムが変わってしまう

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// このような場合、ライフタイム注釈を使う
// これにより、関数の呼び出し元のコードによって、関数の返り値のライフタイムが変わることはなくなる
// つまり、関数の呼び出し元のコードによって、関数の返り値のライフタイムが変わることはなくなる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
