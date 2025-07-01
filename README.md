# 数据结构库

一个用 Rust 实现的常用数据结构和算法库。

## 功能特性

- 线性数据结构:
  - 数组列表
  - 链表
  - 栈
  - 字符串（包含 BF 模式匹配）
  - 图（邻接矩阵）
- 排序算法:
  - 快速排序

## Localization 支持

我们提供多语言版本以支持全球开发者:

- 中文 (zh-CN)
- 英文 (en-US)

## 使用方法

在你的 `Cargo.toml` 中添加:

```toml
[dependencies]
data_structure = "0.1.9"
```

示例代码:

```rust
use data_structure::linear::string::String;
use data_structure::non_linear::graph::AMGraph;

fn main() {
    // 字符串示例
    let s = String::new(&['H', 'e', 'l', 'l', 'o']);
    let sub = String::new(&['l', 'l']);
    println!("匹配位置: {}", s.index_BF(&sub, 0)); // 2

    // 图表示例
    let graph: AMGraph<i32, 3> = AMGraph::from_user_input();
    println!("图顶点: {:?}", graph.vexs_get());
}
```

## 文档说明

运行 `cargo doc --open` 查看完整文档。
