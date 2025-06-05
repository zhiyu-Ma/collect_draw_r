use std::fs::File;
use std::io::Read;
use std::io::Write;

mod stack_collector;
mod framegraph_generator;
mod stack_merger;
mod process_data;

use stack_collector::fetch_and_save_urls;
use framegraph_generator::draw_frame_graph;

use stack_merger::{merge_stacks};
use process_data::process_callstacks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./output/urls.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

    fetch_and_save_urls(urls).await?;  // 保存所有堆栈跟踪到output.json文件

    let input_path = "./output/output.json";  //整理4个进程的数据到txt文件
    let output_path = "./output/processed_stacks.txt";
    process_callstacks(input_path, output_path)?;

    println!("Processed call stacks have been written to {}", output_path);

    // draw_frame_graph("./output/stacks.txt");

    
    Ok(())

}