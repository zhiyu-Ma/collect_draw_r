use std::fs::File;
use std::io::BufReader;
use inferno::flamegraph::{self, Options, Palette};

fn main() {
    let file_path = "stacks.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut options = Options::default();
    options.colors = Palette::Multi(flamegraph::color::MultiPalette::Java);
    // options.colors = inferno::flamegraph::color::Palette::from_str("hot").unwrap();

    let mut output_file = File::create("flamegraph.svg").expect("Failed to create SVG file");
    flamegraph::from_reader(&mut options, reader, &mut output_file).expect("Failed to generate flamegraph");

    println!("Flamegraph generated and saved as flamegraph.svg");
}
