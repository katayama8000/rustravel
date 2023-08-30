use reqwest::Client;
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
struct Address {
    address1: String,
    address2: String,
    address3: String,
    prefcode: String,
    zipcode: String,
}

#[derive(Debug, Deserialize)]
struct ZipCloudResponse {
    status: u32,
    results: Option<Vec<Address>>,
}

#[tokio::main]
async fn main() -> Result<(ZipCloudResponse)> {
    let client = Client::new();
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
        .send()
        .await?;
    let body = response.json::<ZipCloudResponse>().await?;
    println!("{:?}", body);
    Ok((body))
}
