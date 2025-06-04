use std::fs::File;
use std::io::Read;

mod stack_collector;
mod framegraph_generator;

use stack_collector::fetch_and_save_urls;
use framegraph_generator::draw_frame_graph;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("urls.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    fetch_and_save_urls(urls).await?;

    draw_frame_graph("stacks.txt");

    Ok(())
}