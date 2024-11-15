# Conway's Game of Life

Conway's Game of Life is a cellular automaton devised by the British mathematician John Horton Conway in 1970. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the Game of Life by creating an initial configuration and observing how it evolves.

## Rules

The universe of the Game of Life is an infinite, two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, live or dead. Every cell interacts with its eight neighbors, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:

1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

## Implementation

This project provides an implementation of Conway's Game of Life in Rust.

### Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)

### Building and Running

1. Clone this repository:
    ```sh
    git clone https://github.com/oneday1302/conway_game_of_life.git
    cd conway_game_of_life
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the project:
    ```sh
    cargo run --release
    ```

### Usage

The game initializes with a predefined state. You can modify the initial configuration by editing the code in the `main` function.

### Example

```rust
mod grid;

use std::process::Command;
use crate::grid::Grid;

fn clear_screen() {
    Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("Clear-Host")
        .status()
        .unwrap();
}

fn display(mut grid: Grid) {
    let mut generation: u128 = 0;
    loop {
        clear_screen();

        print!("{grid}");
        println!("Generation: {generation}");

        generation += 1;
        grid.tick()
    }
}

fn main() {
    let grid = Grid::new(100, 20);
    display(grid);
}
