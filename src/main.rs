use std::fs::File;
// use std::io::Read;
use std::io::Write;

mod stack_collector;
mod framegraph_generator;
mod stack_merger;
mod process_data;

// use stack_collector::fetch_and_save_urls;
// use framegraph_generator::draw_frame_graph;

// use stack_merger::{merge_stacks, StackTrie};
use process_data::process_callstacks;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut file = File::open("urls.json")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // let urls: Vec<String> = serde_json::from_str(&contents)?;

    // fetch_and_save_urls(urls).await?;

    // draw_frame_graph("stacks.txt");

    //////////////////////////////////////////////////////////////////////////

    // let stacks = vec![
    //     "main;func1;func2;func3",
    //     "main;func1;func2;func4",
    //     "main;func1;func3;func5",
    //     "main;func1;func3;func6",
    // ];

    // let trie = merge_stacks(stacks);

    // let mut output = File::create("merged_stacks.txt")?;
    // for (path, rank_str) in trie.traverse_with_all_stack(&trie.root, Vec::new()) {
    //     writeln!(output, "{} {}", path.join(";"), rank_str)?;
    // }

    // Ok(())
    ////////////////////////////////////////////////////////////////////////////////
    
    let input_path = "response.json";
    let stack_entries = process_callstacks(input_path)?;

    // 打印解析后的数据
    for entry in stack_entries {
        if let Some(frame) = entry.get_frame() {
            println!("Function: {}, File: {}, Line: {}", frame.func, frame.file, frame.lineno);
        }
    }

    Ok(())

}