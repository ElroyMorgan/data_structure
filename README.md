# data_structure

>**注意：**
> 此库只是我用来学习数据结构与算法和Git,Rust，请勿用于实际项目。

`data_structure` 是一个用 Rust 实现的常用数据结构和算法库，提供高性能、类型安全的实现。

## 功能特性

### Err枚举

### 线性数据结构:
- 数组列表 (`linear::array::ArrayList`)
- 静态数组 (`linear::array::SqList`)
- 链表 (`linear::list_node::ListNode`)
- 裸指针链表 (`linear::list_raw::NodeRaw`)
- 枚举链表 (`linear::list::enum_linklist::List`)
- 栈 (`linear::stack::SequentialStack`)
- 队列 (`linear::queue::Queue`)
- 字符串（包含 BF 和 KMP 模式匹配）(`linear::string::String`, `linear::string::index_KMP`)

### 非线性数据结构:
- 图（邻接矩阵）(`non_linear::graph::AMGraph`)
- 树（二叉树）(`non_linear::tree::{BinaryNode, BinaryTree}`)

### 排序算法:
- 快速排序 (`sorting::quick`)

## 使用方法

在 `Cargo.toml` 中添加依赖：
```toml
[dependencies]
data_structure = "0.1.18"
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

// 裸指针链表示例
use data_structure::linear::list_raw::NodeRaw;
let mut raw_list = NodeRaw::new();
raw_list.insert(1, 10).unwrap();

// 枚举链表示例
use data_structure::linear::list::enum_linklist::List;
let mut enum_list = List::new();
enum_list.push(100);

// 栈示例
use data_structure::linear::stack::SequentialStack;
let mut stack = SequentialStack::new(10);
stack.push(1).unwrap();

// 队列示例
use data_structure::linear::queue::Queue;
let mut queue: Queue<i32, 3> = Queue::new();
queue.push(1).unwrap();

// 字符串 BF 匹配示例
use data_structure::linear::string::String;
let s = String::new(&['a','b','c','d','e']);
let sub = String::new(&['c','d']);
assert_eq!(s.index_BF(&sub, 0), 2);

// 字符串 KMP 匹配示例
use data_structure::linear::string::index_KMP;
assert_eq!(index_KMP(&['a','b','c','d','e'], &['c','d']), Some(2));

// 快速排序示例
use data_structure::sorting::quick;
let mut nums = [5,3,1,4,2];
quick(&mut nums, 0, 4);

// 图示例 (需要用户输入，此处仅为示意)
// use data_structure::non_linear::graph::AMGraph;
// let graph: AMGraph<i32, 3> = AMGraph::from_user_input();

// 二叉树示例
use data_structure::non_linear::tree::{BinaryNode, BinaryTree};
let mut tree = BinaryTree::with_root(1);
tree.set_root(BinaryNode::new(1));
```

## 开发状态
- **稳定功能**: 数组列表、静态数组、链表、裸指针链表、枚举链表、栈、队列、字符串匹配算法、快速排序
- **实验功能**: 二叉树(链式调用限制)、图结构(固定顶点数)

## 文档说明
运行以下命令查看完整API文档:
```bash
    cargo doc --open