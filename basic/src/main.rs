use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
// `ParseIntError`を`Err`の型として持つ全ての`Result`のジェネリックエイリアス
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
// 上で定義したエイリアス（この場所特有の`Result`型）を使用
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
// もう一度使用。エイリアスによって再度明記する必要性がない。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    let opt = Some(10);
    let num: Result<i32, &str> = Ok(10);
    match num {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }

    let str: &str = "10";
    let _ret: Result<i32, ParseIntError> = str.parse();
}
