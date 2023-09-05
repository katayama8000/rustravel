use futures::{executor, Future};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[tokio::main]

async fn main() -> Result<()> {
    let ret = add(3, 2).await.unwrap();
    println!("{}", ret);
    let ret2 = add(3, 5).await.expect("msg");
    something_great_async_function().await;
    executor::block_on(something_great_async_function());
    let ret3 = add2(3, 45).await;
    println!("{}", ret3);
    print_result(ret3).await;
    func_move().await;
    let add = |a, b| a + b;
    let result = add(2, 3); // resultには5が格納されます。
    println!("{}", result);
    let name: String = String::from("Alice");
    // func_str(name);
    let greet = || {
        println!("Hello, {}!", name);
    };
    println!("{}", name);
    greet();
    let num = 5;
    let eq = move |z| z == num;
    // fn eq_fn(z: i32) -> bool {
    //     z == x
    // }
    let bool = eq(5);
    println!("{}", bool);
    println!("{}", num);

    let str1 = "aaa".to_string();
    let eq_str = |other_str| other_str == str1;
    println!("{}", eq_str("aaa".to_string()));
    println!("{}", str1);
    let num2 = 10;
    fn_num(num2);
    println!("{}", num2);
    let a = 5;
    let b = a;
    println!("{} {}", a, b);
    Ok(())
}

fn fn_num(num: i32) {
    println!("{}", num)
}
fn func_str(str: String) {
    println!("{}", str)
}

async fn add(x: i32, y: i32) -> Option<i32> {
    Some(x + y)
}

async fn func() -> Result<()> {
    let ret = add(2, 3).await.unwrap();
    Ok(())
}

async fn add2(x: i32, y: i32) -> i32 {
    x + y
}

async fn something_great_async_function() -> i32 {
    let ret = add2(2, 3).await;
    println!("great");
    ret
}

async fn print_result(num: i32) {
    println!("{}", num);
}

fn func_move() -> impl Future<Output = ()> {
    let str = "hello".to_string();
    async move {
        println!("{}", str);
    }
}
