# Sudoku Solver

This Rust crate provides a library for solving Sudoku puzzles.

## Features

- Solve standard 9x9 Sudoku puzzles.
- Validate Sudoku puzzles.

## Requirements

- Rust Compiler (rustc)
- Cargo (Rust's package manager)

## Getting Started

To use this library in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
sudoku_solver = { git = "https://github.com/jeevan0920/sudoku-rs.git" }
```

## Usage

Here's a simple example to demonstrate how to use the Sudoku solver library:

```rust
use sudoku_solver::solve_sudoku;

fn main() {
    let mut puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    if solve_sudoku(&mut puzzle) {
        println!("Solved puzzle:");
        for row in puzzle.iter() {
            for &val in row.iter() {
                print!("{} ", val);
            }
            println!();
        }
    } else {
        println!("No solution found.");
    }
}
```