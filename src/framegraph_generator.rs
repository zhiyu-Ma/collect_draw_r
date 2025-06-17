use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use inferno::flamegraph::{self, Options, Palette};

/// Generates a flamegraph from a stack trace file and saves it as an SVG file.
pub fn draw_frame_graph(file_path: &str) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut options = Options::default();
    options.colors = Palette::Multi(flamegraph::color::MultiPalette::Java);

    let output_path = Path::new("./output/flamegraph222.svg");
    let mut output_file = File::create(output_path).expect("Failed to create SVG file");
    flamegraph::from_reader(&mut options, reader, &mut output_file).expect("Failed to generate flamegraph");

    println!("Flamegraph generated and saved as {}", output_path.display());
}

#[cfg(test)]
use std::fs;
mod tests {
    use super::*;

    #[test]
    fn test_draw_frame_graph() {
        draw_frame_graph("./output/stacks.txt");
        assert!(fs::metadata("./output/flamegraph222.svg").is_ok(), "SVG file should exist");
    }
}
