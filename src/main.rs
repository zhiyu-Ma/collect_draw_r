// use std::fs::File;
// use std::io::BufReader;
// use inferno::flamegraph::{self, Options, Palette};

// fn main() {
//     let file_path = "stacks.txt";
//     let file = File::open(file_path).expect("Failed to open file");
//     let reader = BufReader::new(file);

//     let mut options = Options::default();
//     options.colors = Palette::Multi(flamegraph::color::MultiPalette::Java);
//     // options.colors = inferno::flamegraph::color::Palette::from_str("hot").unwrap();

//     let mut output_file = File::create("flamegraph.svg").expect("Failed to create SVG file");
//     flamegraph::from_reader(&mut options, reader, &mut output_file).expect("Failed to generate flamegraph");

//     println!("Flamegraph generated and saved as flamegraph.svg");
// }

use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use futures::future::join_all; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("urls.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;

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
