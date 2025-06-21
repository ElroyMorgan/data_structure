# Data Structure Library

A Rust library implementing common data structures and algorithms.

## Features

- Linear data structures:
  - Array List
  - Linked List
  - Stack
  - String (with BF pattern matching)
- Sorting algorithms:
  - Quick sort

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
data_structure = "0.1.6"
```

Example usage:

```rust
use data_structure::linear::string::String;

fn main() {
    let s = String::new(&['H', 'e', 'l', 'l', 'o']);
    let sub = String::new(&['l', 'l']);
    println!("Match at: {}", s.index_BF(&sub, 0)); // 2
}
```

## Documentation

Run `cargo doc --open` to view full documentation.
