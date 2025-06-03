use std::fs::File;
use std::io::{BufRead, BufReader};
use inferno::flamegraph::{self, Options};

fn main() {
    // 替换为你的txt文件路径
    let file_path = "stacks.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut options = Options::default();
    // 可以根据需要设置更多选项
    // options.colors = inferno::flamegraph::color::Palette::from_str("hot").unwrap();

    let mut output_file = File::create("flamegraph.svg").expect("Failed to create SVG file");
    flamegraph::from_reader(&mut options, reader, &mut output_file).expect("Failed to generate flamegraph");

    println!("Flamegraph generated and saved as flamegraph.svg");
}
