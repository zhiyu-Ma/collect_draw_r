# 堆栈合并与火焰图绘制工具

[![Rust](https://github.com/yangrudan/collect_draw_r/actions/workflows/rust.yml/badge.svg)](https://github.com/yangrudan/collect_draw_r/actions/workflows/rust.yml)

## 功能介绍

一个高效的命令行工具，用于从多个Web服务节点收集调用堆栈信息，将它们合并，并生成直观的火焰图，以帮助开发者快速定位性能瓶颈。

- **数据收集**: 并发地从多个URL获取调用堆栈数据。
- **智能过滤**: 自动忽略请求失败或无返回数据的节点。
- **堆栈合并**: 使用Trie（字典树）结构高效地合并来自不同节点的堆栈信息。
- **火焰图生成**: 基于合并后的数据，生成可交互的SVG格式火焰图。

## 如何使用

### 1. 构建

本项目使用`rustls`作为TLS后端，避免了对系统OpenSSL库的依赖，简化了编译过程。

```bash
# 克隆项目
git clone https://github.com/yangrudan/collect_draw_r.git
cd collect_draw_r

# 构建项目 (推荐使用release模式以获得更好性能)
cargo build --release
```

构建完成后，可执行文件位于 `./target/release/flame_graph_demo`。

### 2. 配置

在运行程序之前，需要指定要采集数据的目标URL。

1.  在项目根目录下创建一个名为 `output` 的文件夹。
2.  在 `output` 文件夹内，创建一个名为 `urls.json` 的文件。
3.  编辑 `urls.json`，填入一个JSON数组，其中每个字符串都是一个要采集的URL。

**`output/urls.json` 示例:**
```json
[
  "http://10.0.0.1:9922/apis/pythonext/callstack",
  "http://10.0.0.2:9922/apis/pythonext/callstack",
  "http://10.0.0.3:9922/apis/pythonext/callstack"
]
```

### 3. 运行

配置好`urls.json`后，在项目根目录运行程序：

```bash
# 使用cargo直接运行
cargo run --release

# 或者直接运行编译好的二进制文件
./target/release/flame_graph_demo
```

### 4. 查看结果

程序成功运行后，所有输出文件都会保存在 `output` 文件夹中：

- `output.json`: 从所有**成功**的URL收集到的原始堆栈数据（JSON格式）。
- `merged_stacks.txt`: 合并后的堆栈数据，用于生成火焰图的中间文件。
- **`flamegraph.svg`**: **最终生成的火焰图。** 您可以用任何现代浏览器打开此文件进行交互式分析。

## 命令行参数

您可以通过命令行参数来控制程序的行为。使用 `--help` 查看所有可用选项。

```bash
$ ./target/release/flame_graph_demo --help
Usage: flame_graph_demo [OPTIONS]

Options:
  -o, --output-dir <OUTPUT_DIR>
          输出目录路径，默认为 "output"
          
          [default: output]

  -h, --help
          Print help (see more with '--help')

  -V, --version
          Print version
```

- **`-o, --output-dir <OUTPUT_DIR>`**
  - 功能：指定所有输出文件（包括`urls.json`的读取）的存放目录。
  - 默认值：`output`
  - 示例：
    ```bash
    # 将所有文件输出到 my_results 文件夹
    ./target/release/flame_graph_demo --output-dir my_results
    ```

## 错误处理

该工具现在提供了更友好的错误提示。如果遇到问题，程序会明确指出错误原因，例如：
- `output/urls.json` 文件未找到或内容为空。
- 未能从任何URL成功获取数据。

这有助于您快速定位并解决问题。
