use anyhow::Error;

#[tokio::main]
async fn main() {
    let ret = add(2, 3).await;
    let bool1: Option<bool> = Some(false);
    let bool2: anyhow::Result<bool, Error> = Ok(false);
}

async fn add(x: i32, y: i32) -> anyhow::Result<i32, Error> {
    Ok(x + y)
}
