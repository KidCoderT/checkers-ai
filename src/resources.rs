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
    let banners = HashMap::new(); // todo: load the banners

    let piece = load_texture("resources/red.png").await.unwrap();
    pieces.insert(Piece::Red(false), piece);
    pieces.insert(Piece::Red(true), piece);

    let piece = load_texture("resources/blue.png").await.unwrap();
    pieces.insert(Piece::Blue(false), piece);
    pieces.insert(Piece::Blue(true), piece);

    Resources {
        background,
        pieces,
        banners,
    }
}
