use crate::board::Piece;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Resources {
    pub background: Texture2D,
    pub pieces: HashMap<Piece, Texture2D>,
    pub banners: HashMap<Piece, Texture2D>,
}

pub async fn load_resources() -> Resources {
    let background = load_texture("resources/background.png").await.unwrap();
    let mut pieces = HashMap::new();
    let mut banners = HashMap::new();

    let mut piece = load_texture("resources/red.png").await.unwrap();
    pieces.insert(Piece::Red, piece);
    pieces.insert(Piece::RedKing, piece);

    let mut piece = load_texture("resources/blue.png").await.unwrap();
    pieces.insert(Piece::Blue, piece);
    pieces.insert(Piece::BlueKing, piece);

    Resources {
        background,
        pieces,
        banners,
    }
}
