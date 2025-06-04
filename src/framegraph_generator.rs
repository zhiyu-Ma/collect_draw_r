use std::fs::File;
use std::io::BufReader;
use inferno::flamegraph::{self, Options, Palette};

fn draw_frame_graph(file_path: &str) {
    // let file_path = "stacks.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut options = Options::default();
    options.colors = Palette::Multi(flamegraph::color::MultiPalette::Java);

    let mut output_file = File::create("flamegraph.svg").expect("Failed to create SVG file");
    flamegraph::from_reader(&mut options, reader, &mut output_file).expect("Failed to generate flamegraph");

    println!("Flamegraph generated and saved as flamegraph.svg");
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_frame_graph() {
        draw_frame_graph("stacks.txt");
        // 为了简单起见，我们只调用函数。
    }
}
