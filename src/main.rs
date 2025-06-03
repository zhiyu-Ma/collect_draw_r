use std::fs::File;
use std::io::Write;

fn main() {
    let stacks = vec![
        "main; func1; func2",
        "main; func1; func2",
        "main; func1; func3",
        "main; func1; func2",
    ];

    let mut file = File::create("stacks.txt").expect("无法创建文件");
    for stack in stacks {
        writeln!(file, "{}", stack).expect("无法写入文件");
    }

    println!("堆栈数据已保存到 stacks.txt");
}
