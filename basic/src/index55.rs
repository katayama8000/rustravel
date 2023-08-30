use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let res = client.get(url).send().await;
    match res {
        Ok(response) => {
            let body = response.text().await;
            match body {
                Ok(text) => {
                    println!("{}", text);
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Failed to read response body: {}", err);
                    Err(err.into())
                }
            }
        }
        Err(err) => {
            eprintln!("Request failed: {}", err);
            Err(err.into())
        }
    }
}
