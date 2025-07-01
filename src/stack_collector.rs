use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use futures::future::join_all;

/// Fetches JSON data from a list of URLs and saves the combined data to a file.
pub async fn fetch_and_save_urls(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut tasks = Vec::new();
    for url in urls {
        let client = client.clone();
        tasks.push(async move {
            match client.get(&url).send().await {
                Ok(res) => {
                    let body = res.text().await?;
                    let json: Value = serde_json::from_str(&body).unwrap();
                    Ok(Some(json))
                }
                Err(e) => {
                    eprintln!("Error fetching {}: {}", url, e);
                    Ok(None)
                }
            }
        });
    }

    let results: Vec<Result<Option<Value>, reqwest::Error>> = join_all(tasks).await;

    let mut data_list = Vec::new();
    for result in results {
        match result {
            Ok(Some(json)) => data_list.push(json),
            Ok(None) => data_list.push(Value::Array(Vec::new())), // Insert a JSON null value for failed requests
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    let output = serde_json::to_string_pretty(&data_list)?;
    let mut file = File::create("./output/output.json")?;
    file.write_all(output.as_bytes())?;

    println!("Data has been saved to output.json");

    Ok(())
}
