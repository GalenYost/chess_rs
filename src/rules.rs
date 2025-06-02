use crate::board::Board;
use crate::utils::all_positions;
use crate::piece::{Color, Position};

pub fn is_in_check (board: &Board, color: Color) -> bool {
    let king_pos = match board.get_king_pos(color) {
        Some(k) => k,
        None => panic!("No {:?} king on board", color),
    };

    all_positions()
        .filter_map(|pos| board.get(pos.row as i8, pos.col as i8)
            .map(|p| (pos, p)))
        .any(|(pos, p)| {
            p.color != color
                && p
                    .data
                    .legal_moves(pos, p.color, board)
                    .contains(&king_pos)
        })
}

pub fn is_checkmate (board: &mut Board, color: Color) -> bool {
    if !is_in_check(board, color) {
        return false;
    }

    all_positions()
        .filter_map(|from| board.get(from.row as i8, from.col as i8))
        .filter(|p| p.color == color)
        .all(|p| {
            let mut m = p.legal_moves(board);
            filter_moves(board, &mut m, p.pos, color);
            m.is_empty()
        })
}

pub fn is_stalemate (board: &Board, color: Color) -> bool {
    if is_in_check(board, color) {
        return false;
    }

    all_positions()
        .filter_map(|from| board.get(from.row as i8, from.col as i8))
        .filter(|p| p.color == color)
        .all(|p| {
            let mut m = p.legal_moves(board);
            filter_moves(board, &mut m, p.pos, color);
            m.is_empty()
        })
}

pub fn filter_moves (board: &Board, moves: &mut Vec<Position>, from: Position, color: Color) -> () {
    moves.retain(|&mv| {
        let mut b_clone = board.clone();
        if let Some(p) = b_clone.take(from.row as i8, from.col as i8) {
            b_clone.set(mv, Some(p));
        }
        !is_in_check(&b_clone, color)
    });
}
