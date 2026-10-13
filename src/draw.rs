use raylib::prelude::*;
use crate::{board::{Board, Cell}, config::*, piece::Block};

pub fn draw_grid(d: &mut RaylibDrawHandle) {
    for c in 0..COLS {
        for r in 0..ROWS {
            let x = c as i32 * CELL_SIZE;
            let y = r as i32 * CELL_SIZE;
            d.draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, GRID_COLOR);
        }
    }
}

pub fn draw_board(d: &mut RaylibDrawHandle, board: &Board) {
    for (r, row) in board.cells().iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Cell::Filled(piece) = cell {
                let x = c as i32 * CELL_SIZE;
                let y = r as i32 * CELL_SIZE;
                d.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, piece.color());
            }
        }
    }
}

pub fn draw_block(d: &mut RaylibDrawHandle, block: &Block) {
    for p in block.cells() {
        let x = p.x * CELL_SIZE;
        let y = p.y * CELL_SIZE;
        d.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, block.piece.color());
    }
}


