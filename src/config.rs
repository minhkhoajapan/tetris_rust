use raylib::color::Color;

pub const COLS: usize = 10;
pub const ROWS: usize = 20;
pub const CELL_SIZE: i32 = 30;

pub const WINDOW_WIDTH: i32 = COLS as i32 * CELL_SIZE;
pub const WINDOW_HEIGHT: i32 = ROWS as i32 * CELL_SIZE;

pub const BACKGROUND_COLOR: Color = Color::new(0x1E, 0x1F, 0x26, 0xFF);
pub const GRID_COLOR: Color = Color::new(0x3C, 0x3F, 0x4C, 0xFF);
pub const L_PIECE_COLOR: Color = Color::new(0xF9, 0x9a, 0xA9, 0xFF);
pub const J_PIECE_COLOR: Color = Color::new(0x86, 0xBB, 0xE9, 0xFF);
pub const I_PIECE_COLOR: Color = Color::new(0x96, 0x8F, 0xD6, 0xFF);
pub const O_PIECE_COLOR: Color = Color::new(0x7B, 0x3C, 0xAC, 0xFF);
pub const S_PIECE_COLOR: Color = Color::new(0xFA, 0xEE, 0x7C, 0xFF);
pub const T_PIECE_COLOR: Color = Color::new(0x64, 0x6E, 0x8F, 0xFF);
pub const Z_PIECE_COLOR: Color = Color::new(0xF0, 0xA0, 0x6C, 0xFF);

pub const FALL_INTERVAL: f32 = 1.0; // seconds per row
pub const SOFT_DROP_FACTOR: f32 = 5.0;
