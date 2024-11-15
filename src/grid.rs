use std::fmt::{Display, Formatter};
use rand::Rng;
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

pub struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Dead; width]; height];
        cells = Self::rand_init_alive_cells(width, height, cells);
        Grid { height, width, cells }
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for (col, cells) in self.cells.iter().enumerate() {
            for (row, cell) in cells.iter().enumerate() {
                let live_neighbors = self.live_neighbor_count(col, row);
                new_cells[col][row] = match (cell, live_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                };
            }
        }

        self.cells = new_cells;
    }

    fn rand_init_alive_cells(width: usize, height: usize, mut cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        let count_of_alive_cells = (width * height) / 10;
        let mut i = 0;
        while i < count_of_alive_cells {
            let col = rand::thread_rng().gen_range(0..height);
            let row = rand::thread_rng().gen_range(0..width);

            if cells[col][row] == Cell::Alive {
                continue;
            }

            cells[col][row] = Cell::Alive;

            i += 1;
        }

        cells
    }

    fn live_neighbor_count(&self, col: usize, row: usize) -> usize {
        let mut count = 0;

        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                let y = col.wrapping_add(y as usize);
                let x = row.wrapping_add(x as usize);

                if y < self.height && x < self.width && self.cells[y][x] == Cell::Alive {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                let symbol = match cell {
                    Cell::Alive => '🟩',
                    Cell::Dead => ' ',
                };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}