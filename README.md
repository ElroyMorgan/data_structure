# Data Structure Library

A Rust library implementing common data structures and algorithms.

## Features

- Linear data structures:
  - Array List
  - Linked List
  - Stack
  - String (with BF pattern matching)
  - Graph (Adjacency Matrix)
- Sorting algorithms:
  - Quick sort

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
data_structure = "0.1.7"
```

Example usage:

```rust
use data_structure::linear::string::String;
use data_structure::non_linear::graph::AMGraph;

fn main() {
    // String example
    let s = String::new(&['H', 'e', 'l', 'l', 'o']);
    let sub = String::new(&['l', 'l']);
    println!("Match at: {}", s.index_BF(&sub, 0)); // 2

    // Graph example
    let graph: AMGraph<i32, 3> = AMGraph::from_user_input();
    println!("Graph vertices: {:?}", graph.vexs_get());
}
```

## Documentation

Run `cargo doc --open` to view full documentation.
