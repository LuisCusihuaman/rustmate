use std::error::Error;

#[doc = "Enum is used to represent an error that can occur when trying to create a Piece struct from a character.
If the character does not represent a valid chess piece, an InvalidPieceKind error is returned."]
#[derive(Debug, PartialEq)]
pub enum PieceError {
    InvalidPieceKind(char),
}

impl std::fmt::Display for PieceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceError::InvalidPieceKind(c) => write!(f, "Invalid piece kind character: {}", c),
        }
    }
}

impl Error for PieceError {}

#[doc = "Represents the color of a chess piece."]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[doc = "Represents the kind of a chess piece."]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Rook,
    Knight,
    King,
    Bishop,
    Queen,
    Pawn,
}

#[doc = "Represents a chess piece with a color and a kind."]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub position: (usize, usize),
}

impl Piece {
    #[doc = "Returns the color of the piece."]
    pub fn color(&self) -> Color {
        self.color
    }
    #[doc = "Returns the position of the piece."]
    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    #[doc = "Creates and returns a Piece enum variant from the specified character c.
    parameters,
    c A char representing the chess piece to create.
    returns, A Result containing a PieceKind enum variant if the character represents a valid chess piece.
    An InvalidPieceKind error if the character does not represent a valid chess piece."]
    pub fn from_char(c: char, position: (usize, usize)) -> Result<Option<Self>, PieceError> {
        if c == '_' {
            return Ok(None);
        }

        let color = if c.is_lowercase() {
            Color::White
        } else {
            Color::Black
        };

        let kind = match c.to_ascii_uppercase() {
            'R' => PieceKind::King,
            'D' => PieceKind::Queen,
            'A' => PieceKind::Bishop,
            'C' => PieceKind::Knight,
            'T' => PieceKind::Rook,
            'P' => PieceKind::Pawn,
            _ => return Err(PieceError::InvalidPieceKind(c)),
        };

        Ok(Some(Piece {
            color,
            kind,
            position,
        }))
    }

    fn capture_piece_at(&self, position: (usize, usize)) -> bool {
        match self.kind {
            PieceKind::Rook => self.capture_with_rook(position),
            PieceKind::King => self.capture_with_king(position),
            PieceKind::Knight => self.capture_with_knight(position),
            PieceKind::Bishop => self.capture_with_bishop(position),
            PieceKind::Queen => self.capture_with_queen(position),
            PieceKind::Pawn => self.capture_with_pawn(position),
        }
    }
    #[doc = "Checks if the piece can capture another piece at the specified position.
    parameters, position A tuple representing the position of the piece to capture.
    return, true if the piece can capture another piece at the specified position, false otherwise."]
    pub fn can_capture(&self, position: (usize, usize)) -> bool {
        self.capture_piece_at(position)
    }

    fn capture_with_knight(&self, target_position: (usize, usize)) -> bool {
        let (x_cur, y_cur) = self.position;
        let (x_target, y_target) = target_position;
        let x_diff = (x_cur as i32 - x_target as i32).abs();
        let y_diff = (y_cur as i32 - y_target as i32).abs();

        (x_diff == 1 && y_diff == 2) || (x_diff == 2 && y_diff == 1)
    }

    fn capture_with_rook(&self, target_position: (usize, usize)) -> bool {
        // Rooks can only move along a row or a column, not diagonally
        if self.position.0 != target_position.0 && self.position.1 != target_position.1 {
            return false;
        }
        true
    }

    fn capture_with_king(&self, target_position: (usize, usize)) -> bool {
        let (x_cur, y_cur) = self.position;
        let (x_target, y_target) = target_position;
        let dx = (x_target as i32 - x_cur as i32).abs();
        let dy = (y_target as i32 - y_cur as i32).abs();

        if dx > 1 || dy > 1 || (dx == 0 && dy == 0) {
            return false;
        }
        true
    }

    fn capture_with_bishop(&self, target_position: (usize, usize)) -> bool {
        let (x_cur, y_cur) = self.position;
        let (x_target, y_target) = target_position;
        let x_diff = (x_cur as i32 - x_target as i32).abs();
        let y_diff = (y_cur as i32 - y_target as i32).abs();

        x_diff == y_diff
    }

    fn capture_with_queen(&self, target_position: (usize, usize)) -> bool {
        self.capture_with_rook(target_position) || self.capture_with_bishop(target_position)
    }

    fn capture_with_pawn(&self, target_position: (usize, usize)) -> bool {
        let (x1, y1) = self.position;
        let (x2, y2) = target_position;

        let x_diff = (x1 as i32 - x2 as i32).abs();
        let y_diff = (y1 as i32 - y2 as i32).abs();

        let is_diagonal_distance = x_diff == 1 && y_diff == 1;
        let is_valid_direction = match self.color {
            Color::White => y2 > y1,
            Color::Black => y2 < y1,
        };
        is_diagonal_distance && is_valid_direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_can_capture() {
        let king = Piece {
            color: Color::White,
            kind: PieceKind::King,
            position: (0, 0),
        };
        assert!(king.capture_piece_at((0, 1)));
        assert!(king.capture_piece_at((1, 1)));
        assert!(king.capture_piece_at((1, 0)));
        assert!(!king.capture_piece_at((3, 3)));
    }

    #[test]
    fn test_rook_can_capture() {
        let rook = Piece {
            color: Color::White,
            kind: PieceKind::Rook,
            position: (0, 0),
        };
        assert!(rook.capture_piece_at((0, 7)));
        assert!(rook.capture_piece_at((5, 0)));
        assert!(!rook.capture_piece_at((3, 3)));
    }

    #[test]
    fn test_knight_can_capture() {
        let knight = Piece {
            color: Color::White,
            kind: PieceKind::Knight,
            position: (3, 2),
        };
        assert!(knight.capture_piece_at((4, 4)));
        assert!(knight.capture_piece_at((5, 1)));
        assert!(knight.capture_piece_at((5, 3)));
        assert!(!knight.capture_piece_at((1, 2)));
    }

    #[test]
    fn test_bishop_can_capture() {
        let bishop = Piece {
            color: Color::White,
            kind: PieceKind::Bishop,
            position: (2, 2),
        };
        assert!(bishop.capture_piece_at((5, 5)));
        assert!(bishop.capture_piece_at((0, 4)));
        assert!(bishop.capture_piece_at((0, 0)));
        assert!(!bishop.capture_piece_at((0, 2)));
    }

    #[test]
    fn test_queen_can_capture() {
        let queen = Piece {
            color: Color::White,
            kind: PieceKind::Queen,
            position: (2, 2),
        };
        assert!(queen.capture_piece_at((4, 4)));
        assert!(queen.capture_piece_at((4, 2)));
        assert!(queen.capture_piece_at((4, 0)));
        assert!(queen.capture_piece_at((2, 0)));
        assert!(queen.capture_piece_at((2, 4)));
        assert!(queen.capture_piece_at((0, 0)));
        assert!(queen.capture_piece_at((0, 2)));
        assert!(queen.capture_piece_at((0, 4)));
        assert!(!queen.capture_piece_at((7, 6)));
    }

    #[test]
    fn test_pawn_can_capture() {
        let white_pawn = Piece {
            color: Color::White,
            kind: PieceKind::Pawn,
            position: (2, 1),
        };
        assert!(white_pawn.capture_piece_at((1, 2)));
        assert!(white_pawn.capture_piece_at((3, 2)));
        assert!(!white_pawn.capture_piece_at((2, 2)));

        let black_pawn = Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
            position: (2, 6),
        };
        assert!(black_pawn.capture_piece_at((1, 5)));
        assert!(black_pawn.capture_piece_at((3, 5)));
        assert!(!black_pawn.capture_piece_at((2, 5)));
    }

    #[test]
    fn test_invalid_character_for_piece() {
        let result = Piece::from_char('X', (0, 0));
        assert_eq!(result, Err(PieceError::InvalidPieceKind('X')));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid piece kind character: X"
        )
    }

    #[test]
    fn test_piece_from_char_underscore() {
        let piece = Piece::from_char('_', (0, 0)).unwrap();
        assert_eq!(piece, None);
    }

    #[test]
    fn test_valid_character_for_piece() {
        let result = Piece::from_char('R', (0, 0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().kind, PieceKind::King);

        let result = Piece::from_char('D', (0, 0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().kind, PieceKind::Queen);

        let result = Piece::from_char('a', (0, 0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().kind, PieceKind::Bishop);

        let result = Piece::from_char('C', (0, 0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().kind, PieceKind::Knight);

        let result = Piece::from_char('t', (0, 0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().kind, PieceKind::Rook);

        let result = Piece::from_char('P', (0, 0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().kind, PieceKind::Pawn);
    }
}
