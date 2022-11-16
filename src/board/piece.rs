#[derive(Copy, Clone, Hash, Eq, Debug, PartialEq)]
pub enum Piece {
    Blue(bool),
    Red(bool),
    Empty,
}

impl Piece {
    pub fn is_red(&self) -> bool {
        matches!(self, Piece::Red(_))
    }

    pub fn is_blue(&self) -> bool {
        matches!(self, Piece::Blue(_))
    }

    pub fn is_king(&self) -> Option<bool> {
        match self {
            Piece::Blue(true) | Piece::Red(true) => Some(true),
            Piece::Empty => None,
            _ => Some(false),
        }
    }

    pub fn base_form(&self) -> Piece {
        match self {
            Piece::Blue(_) => Piece::Blue(false),
            Piece::Red(_) => Piece::Red(false),
            Piece::Empty => Piece::Empty,
        }
    }

    pub fn empty(&self) -> bool {
        matches!(self, Piece::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::Piece;

    #[test]
    fn check_get_base_form() {
        let test_case = vec![
            (Piece::Red(true), Piece::Red(false)),
            (Piece::Blue(true), Piece::Blue(false)),
            (Piece::Blue(false), Piece::Blue(false)),
            (Piece::Red(false), Piece::Red(false)),
            (Piece::Empty, Piece::Empty),
        ];

        for (i, j) in test_case {
            assert_eq!(i.base_form(), j)
        }
    }

    #[test]
    fn check_is_king() {
        let test_case = vec![
            (Piece::Red(true), Some(true)),
            (Piece::Blue(true), Some(true)),
            (Piece::Red(false), Some(false)),
            (Piece::Blue(false), Some(false)),
            (Piece::Empty, None),
        ];

        for (i, j) in test_case {
            assert_eq!(i.is_king(), j);
        }
    }

    #[test]
    fn check_is_blue_red_or_empty() {
        let test_case = vec![
            (Piece::Red(true).is_red(), true),
            (Piece::Blue(false).is_red(), false),
            (Piece::Blue(true).is_blue(), true),
            (Piece::Red(false).is_blue(), false),
            (Piece::Empty.is_blue(), false),
            (Piece::Empty.is_red(), false),
            (Piece::Empty.empty(), true),
            (Piece::Red(false).empty(), false),
        ];

        for (i, j) in test_case {
            assert_eq!(i, j);
        }
    }
}
