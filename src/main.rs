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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输出目录路径，默认为 "output"
    #[arg(short, long, default_value = "output")]
    output_dir: String,
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
    let output_dir = PathBuf::from(&args.output_dir);
    
    // 确保输出目录存在
    std::fs::create_dir_all(&output_dir)?;

    let urls_path = output_dir.join("urls.json");
    let mut file = File::open(&urls_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    fetch_and_save_urls(urls).await?;

    let input_path = output_dir.join("output.json");
    let output_path = output_dir.join("processed_stacks.txt");
    process_callstacks(input_path.to_str().unwrap(), output_path.to_str().unwrap())?;

    println!("已将处理后的调用栈写入到 {}", output_path.display());

    let file = File::open(&output_path)?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n'); // 保留换行符
    }

    let stacks: Vec<&str> = content.lines().collect();
    let trie = merge_stacks(stacks);

    let merged_stacks_path = output_dir.join("merged_stacks_4ranks.txt");
    let mut output = File::create(&merged_stacks_path)?;
    for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
        writeln!(output, "{} {} 1", path.join(";"), rank_str)?;
    }

    draw_frame_graph(merged_stacks_path.to_str().unwrap());

    Ok(())

}