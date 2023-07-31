use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(); // 1
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
        .send()
        .await?; // 2
    let body = response.text().await?; // 3
                                       // println!("{}", body);

    let json_url = "https://jsonplaceholder.typicode.com/posts";
    let json_response: reqwest::Response = client.get(json_url).send().await?;
    let json_body: String = json_response.text().await?;
    // println!("{}", json_body);
    get_json_data(10).await?;

    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    // println!("body = {:?}", body);
    post_json_data(10).await?;
    Ok(())
}

async fn get_json_data(index: i32) -> Result<()> {
    let client = Client::new(); // 1
    println!("{}", index);
    let json_url = format!("https://jsonplaceholder.typicode.com/posts/{}", index);
    let json_response: reqwest::Response = client.get(json_url).send().await?;
    println!("{:#?}", json_response);

    let json_body: String = json_response.text().await?;
    println!("{}", json_body);
    Ok(())
}

async fn post_json_data(index: i32) -> Result<()> {
    let client = Client::new(); // 1
    let json_url = "https://jsonplaceholder.typicode.com/posts";
    let json_response: reqwest::Response = client.post(json_url).send().await?;
    let json_body: String = json_response.text().await?;
    println!("{}", json_body);
    Ok(())
}
