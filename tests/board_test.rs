use rustmate::board::{Board, Winner};
use rustmate::piece::{Color, Piece, PieceKind};
use std::error::Error;

#[test]
fn test_black_queen_and_white_rook_results_black_captures() -> Result<(), Box<dyn Error>> {
    let mut board = Board::default_board();
    board.next_turn(); // play blacks
    board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::Queen,
        position: (3, 2),
    })?;
    board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Rook,
        position: (5, 4),
    })?;

    let winner: Option<Winner> = board.side_to_move();
    assert_eq!(winner, Some(Winner::Black));
    Ok(())
}

#[test]
fn test_white_bishop_and_black_pawn_results_in_white_captures() -> Result<(), Box<dyn Error>> {
    let mut board = Board::default_board();
    board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Bishop,
        position: (2, 1),
    })?;
    board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::Pawn,
        position: (6, 5),
    })?;

    let winner: Option<Winner> = board.side_to_move();
    assert_eq!(winner, Some(Winner::White));
    Ok(())
}

#[test]
fn test_white_rook_and_black_king_results_in_draw() -> Result<(), Box<dyn Error>> {
    let mut board = Board::default_board();
    board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::King,
        position: (3, 3),
    })?;
    board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Rook,
        position: (3, 4),
    })?;

    let winner: Option<Winner> = board.side_to_move();
    assert_eq!(winner, Some(Winner::Draw));
    Ok(())
}

#[test]
fn test_white_queen_and_black_pawn_results_in_both_lose() -> Result<(), Box<dyn Error>> {
    let mut board = Board::default_board();
    board.place_piece(Piece {
        color: Color::White,
        kind: PieceKind::Queen,
        position: (2, 4),
    })?;
    board.place_piece(Piece {
        color: Color::Black,
        kind: PieceKind::Pawn,
        position: (5, 6),
    })?;
    let winner: Option<Winner> = board.side_to_move();
    assert_eq!(winner, None);
    Ok(())
}
