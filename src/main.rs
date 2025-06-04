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
use std::io::Write;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:9922/apis/pythonext/callstack";

    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    println!("Body: {}", body);

    let json: Value = serde_json::from_str(&body)?;

    let file_path = "response.json";
    let mut file = File::create(file_path)?;
    file.write_all(json.to_string().as_bytes())?;

    println!("JSON response saved to {}", file_path);

    Ok(())
}
