use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use futures::future::join_all; 

pub async fn fetch_and_save_urls(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut tasks = Vec::new();
    for url in urls {
        let client = client.clone();
        tasks.push(async move {
            let res = client.get(&url).send().await?;
            let body = res.text().await?;
            let json: Value = serde_json::from_str(&body).expect("REASON");
            Ok(json)
        });
    }

    let results: Vec<Result<Value, reqwest::Error>> = join_all(tasks).await;

    let mut data_list = Vec::new();
    for result in results {
        match result {
            Ok(json) => data_list.push(json),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    let output = serde_json::to_string_pretty(&data_list)?;
    let mut file = File::create("output.json")?;
    file.write_all(output.as_bytes())?;

    println!("Data has been saved to output.json");

    Ok(())
}