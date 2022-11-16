use crate::board::Piece;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Resources {
    pub background: Texture2D,
    pub pieces: HashMap<Piece, Texture2D>,
    pub banners: HashMap<Piece, Texture2D>,
}

impl Resources {
    pub fn piece_img(&self, piece: &Piece) -> Texture2D {
        self.pieces.get(piece).unwrap().to_owned()
    }
}

pub async fn load_resources() -> Resources {
    let background = load_texture("resources/background.png").await.unwrap();
    let mut pieces = HashMap::new();
    let banners = HashMap::new(); // todo: load the banners

    pieces.insert(
        Piece::Red(false),
        load_texture("resources/red.png").await.unwrap(),
    );
    pieces.insert(
        Piece::Blue(false),
        load_texture("resources/blue.png").await.unwrap(),
    );
    pieces.insert(
        Piece::Red(true),
        load_texture("resources/red_king.png").await.unwrap(),
    );
    pieces.insert(
        Piece::Blue(true),
        load_texture("resources/blue_king.png").await.unwrap(),
    );

    Resources {
        background,
        pieces,
        banners,
    }
}
