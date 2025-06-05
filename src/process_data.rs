use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use serde_json;

// 定义 JSON 数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct Frame {
    pub func: String,
    pub file: String,
    pub lineno: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StackEntry {
    CFrame { c_frame: Frame },
    PyFrame { py_frame: Frame },
}

impl StackEntry {
    pub fn get_frame(&self) -> Option<&Frame> {
        match self {
            StackEntry::CFrame { c_frame } => Some(c_frame),
            StackEntry::PyFrame { py_frame } => Some(py_frame),
        }
    }
}

// 解析 JSON 文件并返回 Vec<StackEntry>
pub fn process_callstacks(input_path: &str) -> io::Result<Vec<StackEntry>> {
    // 读取并解析 JSON 文件
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 解析 JSON 数据
    let data: Vec<serde_json::Value> = serde_json::from_str(&contents)?;

    // 将 JSON 数据转换为 Vec<StackEntry>
    let mut stack_entries = Vec::new();
    for entry in data {
        if let Some(c_frame) = entry.get("CFrame") {
            let c_frame: Frame = serde_json::from_value(c_frame.clone()).unwrap();
            stack_entries.push(StackEntry::CFrame { c_frame });
        } else if let Some(py_frame) = entry.get("PyFrame") {
            let py_frame: Frame = serde_json::from_value(py_frame.clone()).unwrap();
            stack_entries.push(StackEntry::PyFrame { py_frame });
        }
    }

    Ok(stack_entries)
}
