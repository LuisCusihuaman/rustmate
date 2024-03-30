use crate::board::{Board, BoardError};
use crate::piece::{Piece, PieceError};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
#[allow(unused_imports)]
use std::io::Write;
use std::io::{BufRead, BufReader};

#[doc = "
ParserError is an enum that represents possible errors that can occur while parsing a file that describes a chess game.

Possible error types are:

- FileNotExists: indicates that the file does not exist.
- FileIsEmpty: indicates that the file is empty.
- InvalidBoardSize: indicates that the board size specified in the file is not valid.
- NotEnoughTokens: indicates that there are not enough tokens in a line of the file.
- InvalidPosition: indicates that a position specified in the file is not valid.
- PositionOccupied: indicates that a position specified in the file is already occupied for a piece.
- InvalidPiece: indicates that a piece specified in the file is not valid.

This enum derives from Debug and PartialEq traits.
"]
#[derive(Debug, PartialEq)]
pub enum ParserError {
    FileNotExists,
    FileIsEmpty,
    InvalidBoardSize,
    NotEnoughTokens,
    InvalidPosition,
    PositionOccupied,
    InvalidPiece(char),
}

impl From<BoardError> for ParserError {
    fn from(err: BoardError) -> ParserError {
        match err {
            BoardError::InvalidPosition => ParserError::InvalidPosition,
            BoardError::PositionOccupied => ParserError::PositionOccupied,
        }
    }
}

impl From<PieceError> for ParserError {
    fn from(err: PieceError) -> ParserError {
        match err {
            PieceError::InvalidPieceKind(c) => ParserError::InvalidPiece(c),
        }
    }
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileNotExists => write!(f, "File not exists"),
            ParserError::FileIsEmpty => write!(f, "File is empty"),
            ParserError::InvalidBoardSize => write!(f, "Invalid board size"),
            ParserError::NotEnoughTokens => write!(f, "Not enough tokens"),
            ParserError::InvalidPosition => write!(f, "{}", BoardError::InvalidPosition),
            ParserError::PositionOccupied => write!(f, "{}", BoardError::PositionOccupied),
            ParserError::InvalidPiece(c) => write!(f, "{}", PieceError::InvalidPieceKind(*c)),
        }
    }
}

#[doc = "Add a piece to the board at the given position."]
fn add_to_board(board: &mut Board, position: (usize, usize), c: char) -> Result<(), ParserError> {
    if let Some(piece) = Piece::from_char(c, position)? {
        board.place_piece(piece)?;
    }
    Ok(())
}

#[doc = "Check if a file is empty."]
fn file_is_empty(file: &File) -> bool {
    file.metadata().map(|m| m.len()).unwrap_or(0) == 0
}

#[doc = "Check if the board size is valid."]
fn guard_board_size(tokens: usize) -> Result<(), ParserError> {
    if tokens != 8 {
        return if tokens > 8 {
            Err(ParserError::InvalidBoardSize)
        } else {
            Err(ParserError::NotEnoughTokens)
        };
    }
    Ok(())
}

#[doc = "Parse a board from a file given its path"]
pub fn from_path(path: &str) -> Result<Board, ParserError> {
    let file = File::open(path).map_err(|_| ParserError::FileNotExists)?;
    if file_is_empty(&file) {
        return Err(ParserError::FileIsEmpty);
    }
    let reader = BufReader::new(file);
    let mut board = Board::default_board();

    let mut rows = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let piece_line = line.split_whitespace().collect::<String>();

        guard_board_size(piece_line.len())?;
        for (j, c) in piece_line.chars().enumerate() {
            add_to_board(&mut board, (i, j), c)?;
        }
        rows += 1;
    }
    if rows != 8 {
        return Err(ParserError::InvalidBoardSize);
    }
    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::piece::{Color, PieceKind};

    #[test]
    fn test_file_not_exist() -> Result<(), Box<dyn Error>> {
        let result = from_path("test_piola.txt");
        assert_eq!(result, Err(ParserError::FileNotExists));
        Ok(())
    }

    #[test]
    fn test_file_is_empty() -> Result<(), Box<dyn Error>> {
        let input = "";
        let filename = "tests/fixtures/ejemplo05_vacio.txt";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", input).unwrap();

        let result = from_path(filename);

        std::fs::remove_file(filename).unwrap();
        assert_eq!(result, Err(ParserError::FileIsEmpty));
        Ok(())
    }

    #[test]
    fn test_invalid_board_size() -> Result<(), Box<dyn Error>> {
        let input = "_ _ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\
        \n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n";
        let filename = "tests/fixtures/ejemplo05_board_invalido.txt";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", input).unwrap();

        let result = from_path(filename);

        std::fs::remove_file(filename).unwrap();
        assert_eq!(result, Err(ParserError::InvalidBoardSize));
        assert_eq!(result.unwrap_err().to_string(), "Invalid board size");
        Ok(())
    }

    #[test]
    fn test_not_enough_tokens() -> Result<(), Box<dyn Error>> {
        let input = "_ _ _ _\n_ _ _ _\n_ _ _ _\n_ _ _ _\n\
        _ _ _ _\n_ _ _ _\n_ _ _ _\n_ _ _ _\n_ _ _ _\n";
        let filename = "tests/fixtures/ejemplo06_not_enough_tokens.txt";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", input).unwrap();

        let result = from_path(filename);

        std::fs::remove_file(filename).unwrap();
        assert_eq!(result, Err(ParserError::NotEnoughTokens));
        assert_eq!(result.unwrap_err().to_string(), "Not enough tokens");
        Ok(())
    }

    #[test]
    fn test_invalid_token() -> Result<(), Box<dyn Error>> {
        let input = "_ _ _ W _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\
        \n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n";
        let filename = "tests/fixtures/ejemplo07_invalid_token.txt";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", input).unwrap();

        let result = from_path(filename);

        std::fs::remove_file(filename).unwrap();
        assert_eq!(result, Err(ParserError::InvalidPiece('W')));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid piece kind character: W"
        );
        Ok(())
    }

    #[test]
    fn test_valid_token() -> Result<(), Box<dyn Error>> {
        let input = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\
        \n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ d _\n";
        let filename = "tests/fixtures/ejemplo08_valid_token.txt";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", input).unwrap();

        let mut expected_board = Board::default_board();
        let piece = Piece {
            kind: PieceKind::Queen,
            color: Color::White,
            position: (7, 6),
        };
        expected_board.place_piece(piece)?;

        let current_board = from_path(filename)?;

        std::fs::remove_file(filename).unwrap();
        assert_eq!(current_board, expected_board);
        Ok(())
    }
}
