use std::any::Any;

use crate::piece::{PieceData, Color, Name, Position};
use crate::board::Board;
use crate::utils::validate_pos;

#[derive(Clone,Debug)]
pub struct PawnData {
    pub has_moved: bool,
    pub passant_target: Option<Position>
}

impl PieceData for PawnData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        let mut moves = Vec::new();

        let dir = match color {
            Color::White => -1,
            Color::Black => 1,
        };

        let row = pos.row as i8 + dir;
        if validate_pos(row as i8, pos.col as i8) {
            let one_forward = Position { row: row as usize, col: pos.col };
            if board.is_empty_cell(one_forward) {
                moves.push(one_forward);

                if !self.has_moved {
                    let two_forward = pos.row as i8 + 2 * dir;
                    if validate_pos(two_forward as i8, pos.col as i8) {
                        let two_forward = Position { row: two_forward as usize, col: pos.col };
                        if board.is_empty_cell(two_forward) {
                            moves.push(two_forward);
                        }
                    }
                }
            }

            for dc in [-1, 1] {
                let col = pos.col as i8 + dc;
                if validate_pos(row, col) {
                    let diag = Position { row: row as usize, col: col as usize };
                    if board.is_enemy_cell(diag, color) {
                        moves.push(diag);
                    }
                }
            }
        }

        if let Some(target) = self.passant_target {
            moves.push(target);
        }

        moves
    }

    fn on_move (&mut self, from: Position, to: Position, color: Color, board: &mut Board) -> () {
        let start_rank = match color {
            Color::White => 6,
            Color::Black => 1,
        };

        let dir = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        if from.row == start_rank {
            self.calc_passant_target(to, color, board);
        }

        if let Some(passant_target) = self.passant_target {
            if to == passant_target {
                board.take(passant_target.row as i8 + dir, passant_target.col as i8);
            }
        }

        board.clear_passants(color);
        self.moved();
    }
}

impl PawnData {
    pub fn moved (&mut self) -> () {
        self.has_moved = true;
    }

    pub fn calc_passant_target (&self, pos: Position, color: Color, board: &mut Board) -> () {
        let dir = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let row = pos.row as i8 + dir;

        for dc in [-1, 1] {
            let col = pos.col as i8 + dc;
            if validate_pos(row, col) {
                if let Some(p) = board.get_mut(row, col) {
                    if p.name == Name::Pawn && p.color != color {
                        if let Some(pawn_data) = p.data.as_any_mut().downcast_mut::<PawnData>() {
                            pawn_data.passant_target = Some(Position {
                                row: row as usize,
                                col: pos.col,
                            });
                        }
                    }
                }
            }
        }
    }
}
