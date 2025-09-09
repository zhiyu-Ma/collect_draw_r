use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

mod stack_collector;
mod framegraph_generator;
mod stack_merger;
mod process_data;

use stack_collector::fetch_and_save_urls;
use framegraph_generator::draw_frame_graph;
use stack_merger::merge_stacks;
use process_data::process_callstacks;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输出目录路径，默认为 "output"
    #[arg(short, long, default_value = "output")]
    output_dir: String,
}

// Helper function to convert PathBuf to &str, providing a clearer error message.
fn path_to_str<'a>(path: &'a Path) -> Result<&'a str, Box<dyn std::error::Error>> {
    path.to_str().ok_or_else(|| {
        let message = format!("Path contains invalid UTF-8: {}", path.display());
        message.into()
    })
}

/**
 # Steps Description
 
 - collect call stacks from URLs and save them to a json file(output.json)
 - process the call stacks from the json file
 - merge the call stacks and save them to a text file(merged_stacks.txt)
 - draw the frame graph from the merged call stacks(flamegraph.svg)
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let output_dir = PathBuf::from(&args.output_dir);
    
    // 确保输出目录存在
    std::fs::create_dir_all(&output_dir)?;

    // 1. Read URLs from urls.json
    let urls_path = output_dir.join("urls.json");
    if !urls_path.exists() {
        let error_message = format!("Error: Configuration file not found at '{}'. Please create this file and add the target URLs.", urls_path.display());
        return Err(error_message.into());
    }
    let mut file = File::open(&urls_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;
    if urls.is_empty() {
        let error_message = format!("Error: '{}' is empty. Please add target URLs to collect stack data.", urls_path.display());
        return Err(error_message.into());
    }

    // 2. Fetch and save call stacks to output.json
    let collected_stacks_path = output_dir.join("output.json");
    fetch_and_save_urls(urls, path_to_str(&collected_stacks_path)?).await?;
    println!("Collected stacks have been saved to {}", collected_stacks_path.display());

    // 3. Process call stacks directly from the file
    let processed_stacks = process_callstacks(path_to_str(&collected_stacks_path)?)?;
    if processed_stacks.is_empty() {
        let error_message = format!("Error: Failed to collect any stack data from the provided URLs. Please check network connectivity and ensure target services are running.");
        return Err(error_message.into());
    }
    println!("Call stacks processed in memory.");

    // 4. Merge the processed stacks
    let stacks_str: Vec<&str> = processed_stacks.iter().map(AsRef::as_ref).collect();
    let trie = merge_stacks(stacks_str);

    // 5. Write the merged stacks to a file
    let merged_stacks_path = output_dir.join("merged_stacks.txt");
    let mut output = File::create(&merged_stacks_path)?;
    for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
        writeln!(output, "{} {} 1", path.join(";"), rank_str)?;
    }
    println!("Merged stacks saved to {}", merged_stacks_path.display());

    // 6. Draw the flame graph
    let flamegraph_path = output_dir.join("flamegraph.svg");
    draw_frame_graph(path_to_str(&merged_stacks_path)?, path_to_str(&flamegraph_path)?)?;

    Ok(())
}
