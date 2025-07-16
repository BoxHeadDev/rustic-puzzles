# 🧩 Rustic Puzzles

A collection of algorithmic puzzles, programming challenges, and brain teasers implemented in idiomatic Rust.

Includes solutions from:
- [Project Euler](https://projecteuler.net/)
- [Advent of Code (AoC)](https://adventofcode.com/)
- [CSEC problem sets](https://cses.fi/problemset/)
- [LeetCode exercises](https://leetcode.com/)
- Custom puzzles and practice problems


## 🚀 Features

- 🦀 Written entirely in safe, idiomatic Rust
- 📁 Organized by source: `euler/`, `aoc/`, `csec/`, `leetcode/`
- 🧪 Unit tests and optional benchmarking
- 🛠 CLI tool for running puzzles interactively
- 📚 Modular codebase for reusability and testing


## 🧰 Getting Started

### 🔧 Requirements
- [Rust (latest stable)](https://rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### 🔄 Run a puzzle solution
```bash
cargo run -- euler 1
cargo run -- aoc 2022 1
cargo run -- csec a
```


## 📁 Project Structure

```
rustic_puzzles/
├── src/
│   ├── aoc/         # Advent of Code challenges, by year/day
│   ├── euler/       # Project Euler solutions
│   ├── csec/        # CSEC problems
│   ├── leetcode/    # LeetCode-style problems
│   ├── utils/       # Shared helper functions (math, parsing, etc.)
│   ├── cli.rs       # Command-line argument parsing (clap)
│   ├── runner.rs    # Problem dispatch logic
│   └── main.rs      # CLI entry point
├── inputs/          # Optional input files (for AoC etc.)
├── benches/         # Criterion benchmarks (optional)
├── tests/           # Integration tests (optional)
├── Cargo.toml
└── README.md
```


## 🧠 Why This Project?

This repo serves as a:
- **Rust learning resource** — practice ownership, enums, iterators, and error handling
- **Problem-solving archive** — organize and benchmark solutions
- **Toolbox for interviews or competitions** — easily test and reuse patterns


## 📜 License

This project is licensed under the MIT License.


## 🦀 Made with Rust and 💡 curiosity.

