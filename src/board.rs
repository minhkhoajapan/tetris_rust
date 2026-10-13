use crate::config::*;
use crate::piece::{Block, Piece};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Cell {
    #[default]
    Empty,
    Filled(Piece),
}

pub struct Board {
    cells: [[Cell; COLS]; ROWS],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[Cell::Empty; COLS]; ROWS],
        }
    }

    pub fn cells(&self) -> &[[Cell; COLS]; ROWS] {
        &self.cells
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col]  = cell;
    }

    pub fn lock(&mut self, block: &Block) {
        for p in block.cells() {
            self.set(p.y as usize, p.x as usize, Cell::Filled(block.piece));
        }
    }

    pub fn is_valid(&self, block: &Block) -> bool {
        block.cells().iter().all(|p| {
            p.x >= 0 && p.x < COLS as i32 && p.y >= 0 && p.y < ROWS as i32
            && self.cells[p.y as usize][p.x as usize] == Cell::Empty
        })
    }

    pub fn clear_lines(&mut self) -> usize {
        let mut write = ROWS;
        for read in (0..ROWS).rev() {
            let ful = self.cells[read].iter().all(|c| *c != Cell::Empty);
            if !ful {
                write -= 1;
                self.cells[write] = self.cells[read];
            }
        }
        for row in 0..write {
            self.cells[row] = [Cell::Empty; COLS];
        }

        write
    }
}
