use crate::piece::{Position, Color};
use crate::board::Board;

pub fn validate_pos (row: i8, col: i8) -> bool {
    (0..8).contains(&row) && (0..8).contains(&col)
}

pub fn all_positions() -> impl Iterator<Item = Position> {
    (0..8).flat_map(|row| {
        (0..8).map(move |col| Position { row, col })
    })
}

pub fn sliding_moves (board: &Board, from: Position, dirs: &[(i8, i8)], color: Option<Color>) -> Vec<Position> {
    let mut moves = Vec::new();

    for &(dx, dy) in dirs {
        let mut current = from;

        loop {
            if let Some(next_pos) = current.shifted(dx, dy) {
                if board.is_empty_cell(next_pos.row as i8, next_pos.col as i8) {
                    moves.push(next_pos);
                    current = next_pos;
                    continue;
                }
                if let Some(p_color) = color {
                    if board.is_enemy_cell(next_pos.row as i8, next_pos.col as i8, p_color) {
                        moves.push(next_pos);
                    }
                } else {
                    if board.get(next_pos.row as i8, next_pos.col as i8).is_some() {
                        moves.push(next_pos);
                    }
                }
                break;
            } else {
                break;
            }
        }
    }

    moves
}

pub fn step_moves (board: &Board, from: Position, deltas: &[(i8, i8)], color: Color) -> Vec<Position> {
    let mut moves = Vec::new();

    for &(dx, dy) in deltas {
        if let Some(to) = from.shifted(dx, dy) {
            if board.is_empty_cell(to.row as i8, to.col as i8) || board.is_enemy_cell(to.row as i8, to.col as i8, color) {
                moves.push(to);
            }
        }
    }

    moves
}
