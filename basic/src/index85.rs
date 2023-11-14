use std::fs::File;
use std::io::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let x = 4;
    let is_x = |a| a == x;
    println!("{}", is_x(3));

    // let a = async_function();
    // let a = a.await;

    // let b = sample().await.unwrap();

    // someの例
    let some_number = Some(5);
    // matchで取り出す
    let num: usize = match some_number {
        Some(num) => num,
        None => 0,
    };

    // stringの例
    let str1 = "Hello";
    let str2 = String::from("Hello");
    println!("{}", str1);
    println!("{}", str2);
    assert_eq!(str1, str2);
    // メモリの場所を確認
    println!("{:p}", str1);
    println!("{:p}", str2.as_ptr());
    let val: Result<i32, Error> = Ok(1);
    // match
    match val {
        Ok(value) => println!("OK: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    let result = "123".parse::<i32>();
    result.unwrap();
    // if let Ok(num) = result {
    //     println!("数値 {} としてパースできました", num);
    // }

    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    let num = numbers
        .into_iter()
        .filter_map(|num| num.ok())
        .collect::<Vec<i32>>();

    // 関連型の練習

    impl Animal for Cat {
        type Food = Dog;
        fn eat(&self, food: Self::Food) {
            println!("Cat eats Dog {:?}", food);
        }
    }
}

// debug
#[derive(Debug)]
struct Cat;
#[derive(Debug)]
struct Dog;

// 関連型の練習
trait Animal {
    // 関連型
    type Food;
    // 関連関数
    fn eat(&self, food: Self::Food);
}

// test
#[test]
fn test() {
    assert_eq!(1, 1);
    let str1 = "Hello";
    let str2 = String::from("Hello");
    assert_eq!(str1, str2);
}

async fn sample() -> Result<(), reqwest::Error> {
    // APIのエンドポイント
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // 非同期でGETリクエストを送信
    let response = reqwest::get(url).await?;

    // レスポンスのステータスコードを確認
    if response.status().is_success() {
        // JSONデータを非同期で取得
        let body = response.text().await?;
        println!("APIレスポンス: {}", body);
    } else {
        eprintln!(
            "APIリクエストが失敗しました。ステータスコード: {}",
            response.status()
        );
    }

    Ok(())
}

// async await
async fn async_function() {
    println!("Start");
    sleep(Duration::from_secs(1)).await;
    println!("End");
}
