use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use futures::future::join_all;

/// Fetches JSON data from a list of URLs and saves the combined data to a file.
pub async fn fetch_and_save_urls(urls: Vec<String>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let tasks: Vec<_> = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            match client.get(&url).send().await {
                Ok(res) => match res.text().await {
                    Ok(body) => match serde_json::from_str::<Value>(&body) {
                        Ok(json) => json,
                        Err(e) => {
                            eprintln!("Error parsing JSON from {}: {}", url, e);
                            Value::Null
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading response body from {}: {}", url, e);
                        Value::Null
                    }
                },
                Err(e) => {
                    eprintln!("Error fetching {}: {}", url, e);
                    Value::Null
                }
            }
        }
    }).collect();

    let results: Vec<Value> = join_all(tasks).await;

    // Filter out null values from failed requests
    let successful_data: Vec<Value> = results.into_iter().filter(|v| !v.is_null()).collect();

    if successful_data.is_empty() {
        println!("Warning: No data could be collected from any of the URLs. Flamegraph will not be generated.");
    }

    let output = serde_json::to_string_pretty(&successful_data)?;
    let mut file = File::create(output_path)?;
    file.write_all(output.as_bytes())?;

    println!("Successfully saved data from {} URLs to {}", successful_data.len(), output_path);

    Ok(())
}
