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
