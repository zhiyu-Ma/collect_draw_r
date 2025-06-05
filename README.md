# 收集堆栈合并与绘制火焰图

## 介绍

>堆栈合并（Stack Merging）是一种技术，用于将多个调用堆栈中的信息汇总到一起，以便于分析和优化。
>
>火焰图则是一种图形化工具，通过可视化的方式展示函数调用的层次和时间分布，帮助开发者快速定位性能瓶颈。

### 1. 收集数据
**前置条件**: 确保已经启用了性能分析工具Probing网页服务，并且已经生成了性能分析数据.

```bash
❯ cd /home/yang/worksapce/demangle_l                                                                                                                      
❯ source /home/yang/worksapce/probing/venv/bin/activate                                                                                                     ❯ PROBING=1 PROBING_PORT=9922 python main.py     
```

### 2. 堆栈合并
收集多个rank堆栈信息并进行合并.

### 3. 火焰图绘制
定制化火焰图生成.

## Output文件说明

- `urls.json` 为模拟的请求;
- `response.json` 为单进程收集数据;
- `output.json` 为模拟4进程收集的数据;
- `processed_stacks.txt` 为json转换成一维txt;
- `merged_stacks.txt` 为合并后的堆栈信息;
- `mmm.svg` 测试堆栈合并火焰图;
- `flamegraph.svg` 为生成的火焰图;
