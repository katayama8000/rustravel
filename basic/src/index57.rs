use std::fs;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // example().await?;
    let str: String = get_json().await?;
    if let Err(e) = foo() {
        println!("{:?}", e);
    }
    // println!("{}", str);
    let client: Client = Client::new();
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let res = client.get(url).send().await;
    match res {
        Ok(response) => {
            // let body = response.text().await;
            let body = response.json::<User>().await;
            match body {
                Ok(json) => {
                    println!("{:?}", json);
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

async fn example() -> Result<()> {
    let client = reqwest::Client::new();
    let url = "https://example.com";
    let response = client.get(url).send().await?;
    // print!("{:?}", response);
    // ...
    Ok(())
}

async fn get_json() -> Result<String> {
    let client = reqwest::Client::new();
    let url = "https://jsonplaceholder.typicode.com";
    let response = client.get(url).query(&[("todos", "1")]).send().await?;
    let body = response.text().await?;
    Ok(body)
}

async fn get_son() -> Result<User> {
    // example().await?;
    let str: String = get_json().await?;
    // println!("{}", str);
    let client = Client::new();
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let res = client.get(url).send().await;
    match res {
        Ok(response) => {
            // let body = response.text().await;
            let body = response.json::<User>().await;
            match body {
                Ok(json) => {
                    println!("{:?}", json);
                    Ok(json)
                }
                Err(err) => {
                    eprintln!("Failed to read response body: {}", err);
                    Err(err.into())
                }
            }
        }
        Err(err) => {
            println!("Request failed: {}", err);
            Err(err.into())
        }
    }
}

use anyhow::Context;

fn foo() -> Result<()> {
    bar().context("foo error")?;
    Ok(())
}

fn bar() -> Result<()> {
    baz().context("bar error")?;
    Ok(())
}

fn baz() -> Result<()> {
    Err(anyhow::anyhow!("baz error"))
}
