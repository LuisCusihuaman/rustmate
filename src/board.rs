use crate::piece::{Color, Piece};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[doc = "An enum representing the winner of the game."]
#[derive(Debug, PartialEq)]
pub enum Winner {
    White,
    Black,
    Draw,
}

#[doc = "An enum representing errors that can occur while manipulating the board."]
#[derive(Debug, PartialEq)]
pub enum BoardError {
    InvalidPosition,
    PositionOccupied,
}

impl Error for BoardError {}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidPosition => write!(f, "Invalid position"),
            BoardError::PositionOccupied => write!(f, "Position occupied"),
        }
    }
}

#[doc = "A struct representing the chess board."]
#[derive(Debug)]
pub struct Board {
    squares: HashMap<(usize, usize), Option<Piece>>,
    turn: Color,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        if self.turn != other.turn {
            return false;
        }
        let mut pieces = self
            .squares
            .values()
            .filter_map(|v| v.as_ref())
            .collect::<Vec<&Piece>>();
        let mut other_pieces = other
            .squares
            .values()
            .filter_map(|v| v.as_ref())
            .collect::<Vec<&Piece>>();
        pieces.sort_by_key(|p| p.get_position());
        other_pieces.sort_by_key(|p| p.get_position());
        pieces == other_pieces
    }
}

impl Board {
    #[doc = "returns a new Board instance with an empty board and the White player's turn."]
    pub fn default_board() -> Self {
        let mut squares = HashMap::new();
        for x in 0..8 {
            for y in 0..8 {
                squares.insert((x, y), None);
            }
        }
        Board {
            squares,
            turn: Color::White,
        }
    }
    #[doc = "returns a boolean indicating whether a given position on the board is empty or not."]
    fn is_position_empty(&self, position: (usize, usize)) -> bool {
        match self.squares.get(&position) {
            Some(pieces) => pieces.is_none(),
            None => true,
        }
    }
    #[doc = "places a Piece on the board at a given position. If the position is invalid or occupied, it returns an error."]
    pub fn place_piece(&mut self, piece: Piece) -> Result<(), BoardError> {
        let position = piece.get_position();
        match position {
            (x, y) if x > 7 || y > 7 => Err(BoardError::InvalidPosition),
            pos if !self.is_position_empty(pos) => Err(BoardError::PositionOccupied),
            pos => {
                self.squares.insert(pos, Some(piece));
                Ok(())
            }
        }
    }
    #[doc = "returns the Piece at a given position, if any."]
    pub fn piece_at(&self, position: (usize, usize)) -> Option<Piece> {
        *self.squares.get(&position).unwrap_or(&None)
    }
    #[doc = "returns the position of the Piece of a given color, if any."]
    fn get_piece_position_based_on_turn(&self, color: Color) -> Option<(usize, usize)> {
        self.squares.iter().find_map(|(&(x, y), square)| {
            square.and_then(|piece| {
                if piece.color() == color {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
    }
    #[doc = "returns the current turn."]
    pub fn curr_turn(&self) -> Color {
        self.turn
    }
    #[doc = "returns the next turn."]
    fn get_next_turn(&self) -> Color {
        if self.turn == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
    #[doc = "changes the turn to the next one."]
    pub fn next_turn(&mut self) {
        self.turn = self.get_next_turn();
    }

    #[doc = "returns the winner of the game, if there is one."]
    pub fn side_to_move(&self) -> Option<Winner> {
        let curr_piece_position = self
            .get_piece_position_based_on_turn(self.curr_turn())
            .unwrap();
        let next_piece_position = self
            .get_piece_position_based_on_turn(self.get_next_turn())
            .unwrap();
        let curr_piece = self.piece_at(curr_piece_position).unwrap();
        let next_piece = self.piece_at(next_piece_position).unwrap();
        let current_turn_has_capture = curr_piece.can_capture(next_piece_position);
        let next_turn_has_capture = next_piece.can_capture(curr_piece_position);
        match (current_turn_has_capture, next_turn_has_capture, self.turn) {
            (true, false, Color::White) => Some(Winner::White),
            (false, true, Color::White) => Some(Winner::Black),
            (true, false, Color::Black) => Some(Winner::Black),
            (false, true, Color::Black) => Some(Winner::White),
            (true, true, _) => Some(Winner::Draw),
            _ => None,
        }
    }
    #[doc = "returns the character representation of the winner of the game."]
    pub fn finish_game(&self) -> char {
        self.side_to_move()
            .map(|color| match color {
                Winner::White => 'B',
                Winner::Black => 'N',
                Winner::Draw => 'E',
            })
            .unwrap_or('P')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{Color, PieceKind};

    #[test]
    fn test_place_piece() -> Result<(), BoardError> {
        let mut board = Board::default_board();
        let position = (0, 0);
        board.place_piece(Piece {
            color: Color::White,
            kind: PieceKind::King,
            position,
        })?;
        let piece = board.piece_at(position);
        assert_eq!(
            piece,
            Some(Piece {
                color: Color::White,
                kind: PieceKind::King,
                position,
            })
        );
        Ok(())
    }

    #[test]
    fn test_board_are_eq() {
        let mut board1 = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let mut board2 = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        board1
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::King,
                position: (0, 0),
            })
            .unwrap();
        board1
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::King,
                position: (1, 1),
            })
            .unwrap();
        board2
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::King,
                position: (0, 0),
            })
            .unwrap();
        board2
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::King,
                position: (1, 1),
            })
            .unwrap();
        assert_eq!(board1, board2);
    }

    #[test]
    fn test_place_piece_invalid_position() {
        let mut board = Board::default_board();
        let position = (9, 9);
        let result = board.place_piece(Piece {
            color: Color::White,
            kind: PieceKind::King,
            position,
        });
        assert_eq!(result, Err(BoardError::InvalidPosition));
        assert_eq!(result.unwrap_err().to_string(), "Invalid position");
    }

    #[test]
    fn test_place_piece_occupied_position() {
        let mut board = Board::default_board();
        let position = (0, 0);
        board
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::King,
                position,
            })
            .unwrap();

        let result = board.place_piece(Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
            position,
        });

        assert_eq!(result, Err(BoardError::PositionOccupied));
        assert_eq!(result.unwrap_err().to_string(), "Position occupied");
    }

    #[test]
    fn test_next_turn() {
        let mut board = Board::default_board();
        assert_eq!(board.curr_turn(), Color::White);
        board.next_turn();
        assert_eq!(board.curr_turn(), Color::Black);
        board.next_turn();
        assert_eq!(board.curr_turn(), Color::White);
    }

    #[test]
    fn test_get_piece_position_based_on_turn() {
        let mut board = Board::default_board(); //play whites turn
        //board.next_turn();
        board
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::Queen,
                position: (2, 3),
            })
            .unwrap();
        board
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::Rook,
                position: (5, 6),
            })
            .unwrap();
        let curr_position = board.get_piece_position_based_on_turn(Color::White);
        let expected_position = Some((5, 6));
        assert_eq!(curr_position, expected_position);
    }

    #[test]
    fn test_side_to_move_result_black_capture() {
        let mut board = Board::default_board();
        board
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::Queen,
                position: (2, 3),
            })
            .unwrap();
        board
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::Rook,
                position: (5, 6),
            })
            .unwrap();
        board.next_turn();
        assert_eq!(board.side_to_move(), Some(Winner::Black));
    }

    #[test]
    fn test_side_to_move_result_white_capture() {
        let mut board = Board::default_board();
        board
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::Rook,
                position: (0, 0),
            })
            .unwrap();
        board
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::King,
                position: (0, 7),
            })
            .unwrap();
        assert_eq!(board.side_to_move(), Some(Winner::White));
    }

    #[test]
    fn test_side_to_move_result_draw() {
        let mut board = Board::default_board();
        board
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::Rook,
                position: (3, 3),
            })
            .unwrap();
        board
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::King,
                position: (3, 4),
            })
            .unwrap();
        assert_eq!(board.side_to_move(), Some(Winner::Draw));
    }

    #[test]
    fn test_black_wins() {
        let mut board = Board::default_board();
        board
            .place_piece(Piece {
                color: Color::Black,
                kind: PieceKind::Queen,
                position: (2, 3),
            })
            .unwrap();
        board
            .place_piece(Piece {
                color: Color::White,
                kind: PieceKind::Rook,
                position: (5, 6),
            })
            .unwrap();
        board.next_turn();
        let winner = board.finish_game();
        assert_eq!(winner, 'N');
    }
}
