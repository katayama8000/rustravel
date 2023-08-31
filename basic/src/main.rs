type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[tokio::main]
async fn main() -> Result<()> {
    let ret = add(3, 2).await.unwrap();
    println!("{}", ret);
    let ret2 = add(3, 5).await.expect("msg");
    Ok(())
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
