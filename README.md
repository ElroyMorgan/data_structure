# data_structure

>**注意：**
> 此库只是我用来学习数据结构与算法和Git、Rust，请勿用于实际项目。

`data_structure` 是一个用 Rust 实现的常用数据结构和算法库，提供高性能、类型安全的实现。该项目旨在通过实践加深对数据结构原理的理解，并熟悉 Rust 语言特性和内存安全管理。

## 功能特性

### 核心错误类型
- `Err` 枚举：包含 `IndexErr`（索引越界）和 `FullErr`（容器已满）两种常见错误类型

### 线性数据结构:
- **数组列表** (`linear::array::ArrayList`)：动态数组实现，支持自动扩容
- **静态数组** (`linear::array::SqList`)：固定大小数组，提供基础CRUD操作
- **链表** (`linear::list_node::ListNode`)：安全的Box指针实现，支持push/pop和索引访问
- **裸指针链表** (`linear::list_raw::NodeRaw`)：使用裸指针实现，用于学习unsafe代码
- **枚举链表** (`linear::list::enum_linklist::List`)：基于Rust枚举的链表实现
- **栈** (`linear::stack::SequentialStack`)：顺序存储栈，支持push/pop操作
- **队列** (`linear::queue::Queue`)：固定大小队列实现
- **字符串** (`linear::string::String`)：包含BF和KMP模式匹配算法

### 非线性数据结构:
- **图** (`non_linear::graph::AMGraph`)：邻接矩阵实现，支持自定义顶点数据
- **二叉树** (`non_linear::tree::{BinaryNode, BinaryTree}`)：链式存储二叉树，支持左右子节点操作

### 排序算法:
- **快速排序** (`sorting::quick`)：原地排序实现，平均时间复杂度O(nlogn)

## 使用方法

在 `Cargo.toml` 中添加依赖：
```toml
[dependencies]
data_structure = "0.1.20"
```

### 示例代码
```rust
// 数组列表示例
use data_structure::linear::array::ArrayList;
let mut arr = ArrayList::new();
arr.insert(1, 42).unwrap();
assert_eq!(arr.get_element(1), Ok(&42));

// 链表示例
use data_structure::linear::list_node::ListNode;
let mut list = ListNode::new();
list.push(1).push(2);
assert_eq!(list.get(1).unwrap().data, Some(1));

// 二叉树示例
use data_structure::non_linear::tree::{BinaryNode, BinaryTree};
let mut tree = BinaryTree::with_root(1);
tree.root_mut().left_mut(2).right_mut(3);

// 快速排序示例
use data_structure::sorting::quick;
let mut nums = [5,3,1,4,2];
quick(&mut nums, 0, 4);
assert_eq!(nums, [1,2,3,4,5]);
```

## 开发状态
- **稳定功能**: 数组列表、静态数组、链表、栈、队列、字符串匹配算法、快速排序
- **实验功能**: 
  - 二叉树(支持基本节点操作，但缺少遍历算法)
  - 图结构(固定顶点数实现，邻接矩阵存储)
  - 裸指针链表(用于unsafe代码学习，不建议生产环境使用)

## 文档与测试

### 生成API文档
```bash
cargo doc --open
```

### 运行测试
```bash
cargo test
```

## 项目结构