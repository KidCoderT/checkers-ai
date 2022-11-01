use std::ops::Deref;
use macroquad::prelude::*;
use resources::{load_resources, Resources};

const BOARD_OFFSET: u32 = 30;
const BOARD_SIZE: u32 = 640;
const CELL_SIZE: u32 = (BOARD_SIZE / 8) as u32;
const BOARD_BORDER_THICKNESS: i8 = 20;

const NUM_SQUARES_TO_EDGE: [[i32; 4]; 64] = [[0; 4]; 64];
const DIRECTIONAL_OFFSET: (i8, i8, i8, i8) = (7, 9, -7, -9);

const WHITE_SQUARES: Color = Color::new(1.00, 1.00, 1.00, 1.00);
const BLACK_SQUARES: Color = Color::new(0.09, 0.18, 0.21, 1.00);

pub mod utils;
pub mod board;
mod resources;
mod ai;

fn window_conf() -> Conf {
    Conf {
        window_title: "CheckersAI".to_owned(),
        window_width: 1200 as i32,
        window_height: (BOARD_SIZE + BOARD_OFFSET * 2) as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

fn draw_board() {
    for file in 0..8 {
        for rank in 0..8 {
            let is_white = (file + rank) % 2 == 0;
            let color: Color = {
                if is_white {
                    WHITE_SQUARES
                } else {
                    BLACK_SQUARES
                }
            };
            draw_rectangle(
                (BOARD_OFFSET + (file * CELL_SIZE)) as f32,
                (BOARD_OFFSET + (rank * CELL_SIZE)) as f32,
                CELL_SIZE as f32,
                CELL_SIZE as f32,
                color,
            );
        }
    }
}

fn draw_scaled_img(img: Texture2D, x: f32, y: f32, scale: f32) {
    let img_data = img.get_texture_data();

    let scaled_width = scale * (img_data.width() as f32);
    let scaled_height = scale * (img_data.height() as f32);

    // let color = 

    draw_texture_ex(
        img,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(scaled_width, scaled_height)),
            ..Default::default()
        }
    )
}

fn draw_pieces(board: [board::Position; 64], resources: Resources) {
    for index in 0..64 {
        let piece = board[index].contains;

        let img = match piece {
            board::Piece::Empty => { continue },
            _ => resources.pieces.get(&piece).unwrap(),
        };

        let x: f32 = BOARD_OFFSET as f32 + CELL_SIZE as f32 * (index as f32 % 8.);
        let y: f32 = BOARD_OFFSET as f32 + CELL_SIZE as f32 * (index as f32 / 8.);

        draw_scaled_img(img.clone(), x, y, 0.4)
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let manager = board::Manager::new();
    let resources = load_resources().await;

    loop {
        clear_background(Color::from_rgba(254, 241, 208, 255));
        draw_texture(&resources.background, 0f32, 0f32, WHITE);
        draw_board();
        draw_pieces(manager.board, resources);

        next_frame().await
    }
}
