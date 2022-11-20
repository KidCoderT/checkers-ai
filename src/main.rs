use macroquad::prelude::*;
use resources::{load_resources, Resources};

const BOARD_OFFSET: f32 = 30f32;
const BOARD_SIZE: f32 = 640f32;
const CELL_SIZE: f32 = (BOARD_SIZE / 8f32) as f32;

const PIECE_SCALE: f32 = 0.4f32;

const WHITE_SQUARES: Color = Color::new(1.00, 1.00, 1.00, 1.00);
const BLACK_SQUARES: Color = Color::new(0.09, 0.18, 0.21, 1.00);

const END_COLOR: Color = Color::new(0.96, 0.81, 0.16, 1.00);
const KILL_COLOR: Color = Color::new(0.96, 0.16, 0.16, 1.00);

const CIRCLE_RADIUS: f32 = 10.00;

mod ai;
pub mod board;
mod resources;
pub mod utils;

fn window_conf() -> Conf {
    Conf {
        window_title: "CheckersAI".to_owned(),
        window_width: 1200i32,
        window_height: (BOARD_SIZE + BOARD_OFFSET * 2f32) as i32,
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
                BOARD_OFFSET + (file as f32 * CELL_SIZE),
                BOARD_OFFSET + (rank as f32 * CELL_SIZE),
                CELL_SIZE as f32,
                CELL_SIZE as f32,
                color,
            );
        }
    }
}

fn draw_scaled_img(img: Texture2D, x: f32, y: f32, scale: f32, should_center: bool) {
    let img_data = img.get_texture_data();

    let scaled_width = scale * (img_data.width() as f32);
    let scaled_height = scale * (img_data.height() as f32);

    let offset_x = if should_center {
        scaled_width / 2f32
    } else {
        0.0
    };
    let offset_y = if should_center {
        scaled_height / 2f32
    } else {
        0.0
    };

    draw_texture_ex(
        img,
        x - offset_x,
        y - offset_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(scaled_width, scaled_height)),
            ..Default::default()
        },
    )
}

fn draw_pieces(board: &[board::Piece; 64], active_index: &Option<usize>, resources: &Resources) {
    let should_not_draw = match active_index {
        Some(index) => index.to_owned(),
        None => 5000usize,
    };

    // todo: make more efficient

    for (index, piece) in board.iter().enumerate() {
        if index == should_not_draw {
            continue;
        }

        let img = match piece {
            board::Piece::Empty => continue,
            _ => resources.piece_img(piece),
        };

        let x: f32 = BOARD_OFFSET + (CELL_SIZE * ((index % 8) as f32 + 0.5));
        let y: f32 = BOARD_OFFSET + (CELL_SIZE * ((index / 8) as f32 + 0.5));

        if piece.is_king().unwrap() {
            let offset = (16f32 * PIECE_SCALE) / 2f32;
            draw_scaled_img(img, x, y + offset, PIECE_SCALE, true);
            draw_scaled_img(img, x, y - offset, PIECE_SCALE, true);
        } else {
            draw_scaled_img(img, x, y, PIECE_SCALE, true);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let resources = load_resources().await;
    let mut manager = board::Manager::new();
    let mut active_index: Option<usize> = None;
    let mut active_moves: Vec<board::Move> = Vec::new();

    loop {
        clear_background(Color::from_rgba(254, 241, 208, 255));
        draw_texture(resources.background, 0f32, 0f32, WHITE);

        draw_board();

        // todo: add indicator for the last move

        draw_pieces(&manager.board, &active_index, &resources);

        let (mx, my) = mouse_position();
        let inside_board: bool = {
            let x = mx - BOARD_OFFSET as f32;
            let y = my - BOARD_OFFSET as f32;

            x > 0f32 && x < BOARD_SIZE as f32 && y > 0f32 && y < BOARD_SIZE as f32
        };

        if is_mouse_button_pressed(MouseButton::Left) {
            let x = mx - BOARD_OFFSET as f32;
            let y = my - BOARD_OFFSET as f32;

            if x > 0f32 && x < BOARD_SIZE && y > 0f32 && y < BOARD_SIZE {
                let index = (y / CELL_SIZE) as usize * 8 + (x / CELL_SIZE) as usize;

                if !manager.board[index].is_empty() {
                    active_moves = manager.piece_moves(index);
                    active_index = Some(index);
                }
            }
        }

        if let Some(drag_index) = active_index {
            if is_mouse_button_down(MouseButton::Left) {
                let piece = &manager.board[drag_index as usize];

                let img = match piece {
                    board::Piece::Empty => {
                        panic!("theres an error!")
                    }
                    _ => resources.piece_img(piece),
                };

                for index in active_moves.iter().map(|x| x.end) {
                    let x: f32 = BOARD_OFFSET + (CELL_SIZE * ((index % 8) as f32 + 0.5));
                    let y: f32 = BOARD_OFFSET + (CELL_SIZE * ((index / 8) as f32 + 0.5));

                    draw_circle(x, y, CIRCLE_RADIUS, END_COLOR);
                }

                for (kill_index, _) in active_moves.iter().flat_map(|x| &x.kills) {
                    let x: f32 = BOARD_OFFSET + (CELL_SIZE * ((kill_index % 8) as f32 + 0.5));
                    let y: f32 = BOARD_OFFSET + (CELL_SIZE * ((kill_index / 8) as f32 + 0.5));

                    draw_circle(x, y, CIRCLE_RADIUS, KILL_COLOR);
                }


                for through_index in active_moves.iter().flat_map(|x| &x.through) {
                    let x: f32 = BOARD_OFFSET + (CELL_SIZE * ((through_index % 8) as f32 + 0.5));
                    let y: f32 = BOARD_OFFSET + (CELL_SIZE * ((through_index / 8) as f32 + 0.5));

                    draw_circle(x, y, CIRCLE_RADIUS, END_COLOR);
                }

                draw_scaled_img(img, mx, my, 0.4, true)
            }

            if is_mouse_button_released(MouseButton::Left) {
                let x = mx - BOARD_OFFSET as f32;
                let y = my - BOARD_OFFSET as f32;

                if inside_board {
                    let index = (y / CELL_SIZE) as usize * 8 + (x / CELL_SIZE as f32) as usize;

                    if let Some(move_index) = active_moves.iter().position(|x| x.end == index) {
                        manager.play_move(active_moves[move_index].clone())
                    }
                }

                active_index = None;
                active_moves.clear()
            }
        }

        next_frame().await
    }
}
