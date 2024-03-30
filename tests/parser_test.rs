use rustmateboard::Board;
use rustmateparser;
use rustmatepiece::{Color, Piece, PieceKind};
use std::error::Error;

#[test]
fn test_black_queen_and_white_tower_in_file() -> Result<(), Box<dyn Error>> {
    let mut expected_board = Board::default_board();
    expected_board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::Queen,
        position: (2, 3),
    })?;
    expected_board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Rook,
        position: (5, 6),
    })?;

    let board = parser::from_path("tests/fixtures/ejemplo01.txt")?;
    assert_eq!(board, expected_board);
    Ok(())
}

#[test]
fn test_black_pawn_and_white_bishop_in_file() -> Result<(), Box<dyn Error>> {
    let mut expected_board = Board::default_board();
    expected_board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::Pawn,
        position: (2, 6),
    })?;
    expected_board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Bishop,
        position: (6, 2),
    })?;

    let board = parser::from_path("tests/fixtures/ejemplo02.txt")?;
    assert_eq!(board, expected_board);
    Ok(())
}

#[test]
fn test_black_king_and_white_rook_in_file() -> Result<(), Box<dyn Error>> {
    let mut expected_board = Board::default_board();
    expected_board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::King,
        position: (2, 2),
    })?;
    expected_board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Rook,
        position: (3, 2),
    })?;

    let board = parser::from_path("tests/fixtures/ejemplo03.txt")?;
    assert_eq!(board, expected_board);
    Ok(())
}

#[test]
fn test_black_pawn_and_white_queen_in_file() -> Result<(), Box<dyn Error>> {
    let mut expected_board = Board::default_board();
    expected_board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::Pawn,
        position: (1, 5),
    })?;
    expected_board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Queen,
        position: (3, 2),
    })?;

    let board = parser::from_path("tests/fixtures/ejemplo04.txt")?;
    assert_eq!(board, expected_board);
    Ok(())
}
