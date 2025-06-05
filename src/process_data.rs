use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
enum Frame {
    CFrame(CFrame),
    PyFrame(PyFrame),
}

#[derive(Debug, Deserialize, Serialize)]
struct CFrame {
    file: String,
    func: String,
    ip: String,
    lineno: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PyFrame {
    file: String,
    func: String,
    lineno: u32,
    locals: serde_json::Value,
}

// 解析 JSON 文件并处理调用栈
pub fn process_callstacks(input_path: &str, output_path: &str) -> io::Result<()> {
    // 读取并解析 JSON 文件
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 解析 JSON 数据
    let frames:  Vec<Vec<Frame>> = serde_json::from_str(&contents)?;

    // 处理调用栈
    let mut out_stacks = Vec::new();
    for (i, trace) in frames.iter().enumerate() {
        let mut local_stack = Vec::new();
        for frame in trace {
            match frame {
                Frame::CFrame(cframe) => {
                    println!("  CFrame:");
                    println!("    File: {:?}", cframe.file);
                    println!("    Function: {}", cframe.func);
                    println!("    IP: {}", cframe.ip);
                    println!("    Line: {}", cframe.lineno);
                }
                Frame::PyFrame(pyframe) => {
                    println!("  PyFrame:");
                    println!("    File: {}", pyframe.file);
                    println!("    Function: {}", pyframe.func);
                    println!("    Line: {}", pyframe.lineno);
                    println!("    Locals: {:?}", pyframe.locals);
                }
            }
            local_stack.push(frame.clone());
        }
        local_stack.reverse();
        out_stacks.push(local_stack);
    }

    // 准备输出数据
    let mut prepare_stacks = Vec::new();
    for rank in out_stacks {
        if !rank.is_empty() {
            let data = rank
                .iter()
                .map(|entry| match entry {
                    Frame::CFrame(frame) => format!("{} ({}:{})", frame.func, frame.file, frame.lineno),
                    Frame::PyFrame(frame) => format!("{} ({}:{})", frame.func, frame.file, frame.lineno),
                })
                .collect::<Vec<String>>()
                .join(";");
            prepare_stacks.push(data);
        }
    }

    // 将堆栈数据写入输出文件
    let mut output_file = File::create(output_path)?;
    for stack in prepare_stacks {
        writeln!(output_file, "{}", stack)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn test_process_callstacks() {
        let input_path = "./output/output.json"; 
        let output_path = "./output/processed_stacks.txt";
        let result = process_callstacks(input_path, output_path);
        assert!(result.is_ok(), "Processing call stacks should succeed");
        assert!(std::fs::metadata(output_path).is_ok(), "Output file should exist");
        let output_content = std::fs::read_to_string(output_path).expect("Failed to read output file");
        assert!(!output_content.is_empty(), "Output file should not be empty"); 
    }
}
