use std::fs::File;
use std::io::Read;

mod stack_collector;

use stack_collector::fetch_and_save_urls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("urls.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    fetch_and_save_urls(urls).await?;

    Ok(())
}