#![allow(nunused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev()->Result<()>{
    let hc = httpc_test::new_client("http://localhost:3000");
    let res = hc.get("/").await?.print().await?;
    Ok(())
}