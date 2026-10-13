mod piece;
mod board;
mod config;
mod draw;

use raylib::prelude::*;
use crate::{board::{Board}, config::*, piece::{ALL, Block, Pos}};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Tetris")
        .build();

    rl.set_target_fps(60);

    let mut board = Board::new();

    let mut current_block = random_block(&rl);

    let mut fall_timer = 0.0; 

    // main game loop
    let mut game_over = false;
    while !rl.window_should_close() {
        if !game_over {
            handle_input(&mut rl, &board, &mut current_block);
            let interval = if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                FALL_INTERVAL / SOFT_DROP_FACTOR
            } else {
                FALL_INTERVAL
            };
            fall_timer += rl.get_frame_time();
            if fall_timer >= interval {
                fall_timer -= interval;
                let mut candidate = current_block;
                candidate.move_vertical(1);
                if board.is_valid(&candidate) {
                    current_block = candidate;
                } else {
                    board.lock(&current_block);
                    board.clear_lines();
                    current_block = random_block(&rl);
                    if !board.is_valid(&current_block) {
                        game_over = true;
                    }
                }
            }
        } else if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            game_over = false;
            fall_timer = 0.0;
            board = Board::new();
            current_block = random_block(&rl);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);
        draw::draw_board(&mut d, &board);
        draw::draw_block(&mut d, &current_block);
        draw::draw_grid(&mut d);
        
        if game_over {
            d.draw_text("GAME OVER - press Enter", 20, WINDOW_HEIGHT / 2, 20, Color::RED);
        }
    }
}

fn handle_input(rl: &mut RaylibHandle, board: &Board, current_block: &mut Block) {
    if let Some(key) = rl.get_key_pressed() {
        match key {
            KeyboardKey::KEY_LEFT => {
                let mut candidate = *current_block;
                candidate.move_horizontal(-1);
                if board.is_valid(&candidate) {
                    *current_block = candidate
                }
            }
            KeyboardKey::KEY_RIGHT => {
                let mut candidate = *current_block;
                candidate.move_horizontal(1);
                if board.is_valid(&candidate) {
                    *current_block = candidate
                }
            }
            KeyboardKey::KEY_SPACE => {
                let mut rotated = *current_block;
                rotated.rotate_clockwise();
                for dx in [0, -1, 1, -2, 2] {
                    let mut candidate = rotated;
                    candidate.move_horizontal(dx);
                    if board.is_valid(&candidate) {
                        *current_block = candidate;
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}

fn random_block(rl: &RaylibHandle) -> Block {
    let i: i32 = rl.get_random_value(0..6);
    Block { piece: ALL[i as usize], pos: Pos{x:4, y:1}, rotation: 0}
}

