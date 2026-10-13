use raylib::color::Color;
use crate::config::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}

const fn make_pos(x: i32, y: i32) -> Pos {
    Pos{x, y}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    L, J, I, O, S, T, Z
}

impl Piece {
    // return the relative positions of the pieces of a block relative to its center
    pub fn shape(self, rotation: u8) -> [Pos; 4] {
        let table = match self {
            Piece::L => &L_SHAPES,
            Piece::J => &J_SHAPES,
            Piece::I => &I_SHAPES,
            Piece::O => &O_SHAPES,
            Piece::S => &S_SHAPES,
            Piece::T => &T_SHAPES,
            Piece::Z => &Z_SHAPES,
             
        };
        table[(rotation % 4) as usize]
    }

    // return the color of a given PIECE
    pub fn color(self) -> Color {
        match self {
            Piece::L => L_PIECE_COLOR,
            Piece::J => J_PIECE_COLOR,
            Piece::I => I_PIECE_COLOR,
            Piece::O => O_PIECE_COLOR,
            Piece::S => S_PIECE_COLOR,
            Piece::T => T_PIECE_COLOR,
            Piece::Z => Z_PIECE_COLOR
        }
    }
} 

pub const ALL: [Piece; 7] = [
    Piece::L,
    Piece::J,
    Piece::I,
    Piece::O,
    Piece::S,
    Piece::T,
    Piece::Z
];

const L_SHAPES: [[Pos;4];4] = rotations([make_pos(-1, 0), make_pos(0, 0), make_pos(1, 0), make_pos(1,-1)]);
const J_SHAPES: [[Pos;4];4] = rotations([make_pos(-1, -1), make_pos(-1, 0), make_pos(0, 0), make_pos(1, 0)]);
const I_SHAPES: [[Pos;4];4] = rotations([make_pos(-1, 0), make_pos(0, 0), make_pos(1, 0), make_pos(2, 0)]);
const O_SHAPES: [[Pos;4];4] = rotations([make_pos(0,0), make_pos(0, 1), make_pos(1, 0), make_pos(1, 1)]);
const S_SHAPES: [[Pos;4];4] = rotations([make_pos(-1,0), make_pos(0,0), make_pos(0,-1), make_pos(1, -1)]);
const T_SHAPES: [[Pos;4];4] = rotations([make_pos(-1, 0), make_pos(0,0), make_pos(0,-1), make_pos(1,0)]);
const Z_SHAPES: [[Pos;4];4] = rotations([make_pos(-1,-1), make_pos(0,-1), make_pos(0,0), make_pos(1,0)]);

const fn rotate_cw(p: Pos) -> Pos {
    make_pos(-p.y, p.x)
}

const fn rotations(base: [Pos; 4]) -> [[Pos;4];4] {
    let mut res = [base; 4];
    let mut i = 1;
    while i < 4 {
        let mut j = 0;
        while j < 4 {
            res[i][j] = rotate_cw(res[i-1][j]); 
            j += 1
        }
        i += 1
    }

    res
}

// ------------------------------------------------------------------------
/*
    Implementing a block for tetris
*/
#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub piece: Piece,
    pub pos: Pos,
    pub rotation: u8 // 0..4, number of clockwise quarter
}

impl Block {
    // return the absolute coordinates of the block on the grid
    pub fn cells(self) -> [Pos; 4] {
        self.piece.shape(self.rotation).map(|o| {
            Pos{x: self.pos.x + o.x, y: self.pos.y + o.y}
        })
    }

    pub fn move_horizontal(&mut self, unit: i32) {
        self.pos.x += unit;
    }

    pub fn move_vertical(&mut self, unit: i32) {
        self.pos.y += unit;
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotation += 1;
    }
}



