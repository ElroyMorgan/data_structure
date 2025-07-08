# data_structure

>**注意：**
> 此库只是我用来学习数据结构与算法和Git,Rust，请勿用于实际项目。

`data_structure` 是一个用 Rust 实现的常用数据结构和算法库，提供高性能、类型安全的实现。

## 功能特性

### Err枚举

### 线性数据结构:
- 数组列表 (`linear::array::ArrayList`)
- 静态数组 (`data_structure::linear::array::SqList `)
- 链表 (`linear::list_node::ListNode`)
- 栈 (`linear::stack::SequentialStack`)
- 队列 (`linear::queue::Queue`)
- 字符串（包含 BF 和 KMP 模式匹配）(`linear::string::String`)

### 非线性数据结构:
- 图（邻接矩阵）(`non_linear::graph::AMGraph`)
- 树（二叉树）(`non_linear::tree::{BinaryNode, BinaryTree}`)

### 排序算法:
- 快速排序 (`sorting::quick`)

## 使用方法

在 `Cargo.toml` 中添加依赖：
```toml
[dependencies]
data_structure = "0.1.14"
```

### 示例代码
```
// 数组列表示例
use data_structure::linear::array::ArrayList;
let mut arr = ArrayList::new();
arr.insert(1, 42).unwrap();

// 链表示例
use data_structure::linear::list_node::ListNode;
let mut list = ListNode::new();
list.push(1).push(2);

// 快速排序示例
use data_structure::sorting::quick;
let mut nums = [5,3,1,4,2];
quick(&mut nums, 0, 4);
```

## 开发状态
- **稳定功能**: 数组列表、链表、栈、队列、字符串匹配算法、快速排序
- **实验功能**: 二叉树(链式调用限制)、图结构(固定顶点数)

## 文档说明
运行以下命令查看完整API文档:
```bash
    cargo doc --open
```