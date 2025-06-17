use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::io::Write;
use std::path::PathBuf;
use clap::Parser;

mod stack_collector;
mod framegraph_generator;
mod stack_merger;
mod process_data;

use stack_collector::fetch_and_save_urls;
use framegraph_generator::draw_frame_graph;
use stack_merger::merge_stacks;
use process_data::process_callstacks;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the URLs JSON file
    #[arg(short, long, default_value = "./output/urls.json")]
    urls_file: PathBuf,

    /// Path to the output directory
    #[arg(short, long, default_value = "./output")]
    output_dir: PathBuf,
}

/**
 # Steps Description
 
 - collect call stacks from URLs and save them to a json file(output.json)
 - process the call stacks and save them to a text file(processed_stacks.txt)
 - merge the call stacks and save them to a text file(merged_stacks_4ranks.txt)
 - draw the frame graph from the merged call stacks(frame_graph.png)
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    let mut file = File::open(&args.urls_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    fetch_and_save_urls(urls).await?;

    let output_json = args.output_dir.join("output.json");
    let processed_stacks = args.output_dir.join("processed_stacks.txt");
    process_callstacks(output_json.to_str().unwrap(), processed_stacks.to_str().unwrap())?;

    println!("Processed call stacks have been written to {}", processed_stacks.display());

    let file = File::open(&processed_stacks)?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    let stacks: Vec<&str> = content.lines().collect();
    let trie = merge_stacks(stacks);

    let merged_stacks = args.output_dir.join("merged_stacks_4ranks.txt");
    let mut output = File::create(&merged_stacks)?;
    for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
        writeln!(output, "{} {} 1", path.join(";"), rank_str)?;
    }

    draw_frame_graph(merged_stacks.to_str().unwrap());

    Ok(())
}