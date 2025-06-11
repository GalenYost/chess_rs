use crate::piece::rook::RookData;
use crate::rules::{filter_moves, is_in_check};
use crate::utils::{step_moves, sliding_moves};
use crate::piece::{Color, MoveMeta, Name, Piece, PieceData, Position};
use crate::board::Board;

use std::any::Any;

static STEPS: &[(i8, i8)] = &[
    (1,0), (-1,0),
    (1,-1), (0,-1), (-1,-1),
    (-1,1), (0,1), (1,1),
];

static CASTLE_DIRS: &[(i8, i8)] = &[
    (1, 0),
    (-1, 0),
];

#[derive(Clone,Debug)]
pub struct KingData {
    pub has_moved: bool,
}

impl PieceData for KingData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        let mut moves = step_moves(board, pos, STEPS, color);
        if self.has_moved { return moves; }

        let sliding_moves = sliding_moves(board, pos, CASTLE_DIRS, None);
        for mv in sliding_moves.iter() {
            let mut b_clone = board.clone();
            if let Some(king_p) = b_clone.take(pos.row as i8, pos.col as i8) {
                b_clone.set(mv.row as i8, mv.col as i8, Some(king_p));
                if is_in_check(&b_clone, color) { break; }
            }

            if let Some(p) = board.get(mv.row as i8, mv.col as i8) {
                if p.name != Name::Rook || p.color != color { continue; }
                if let Some(data) = p.data.as_any().downcast_ref::<RookData>() {
                    if data.has_moved { continue; }
                    let d_col = (mv.col as i8 - pos.col as i8).signum();
                    if let Some(castle_target) = pos.shifted(d_col*2, 0) {
                        moves.push(castle_target);
                    }
                }
            }
        }
        
        moves
    }

    fn on_move (&mut self, from: Position, to: Position, color: Color, board: &mut Board) -> Option<MoveMeta> {
        let delta_col = to.col as i8 - from.col as i8;
        let is_castle = delta_col.abs() == 2;
        self.moved();
        Some(MoveMeta {
            piece_name: Name::King,
            piece_color: color,
            from,
            to,
            capture: board.get(to.row as i8, to.col as i8).is_some(),
            promotion: None,
            castle: !self.has_moved && is_castle
        })
    }
}

impl KingData {
    pub fn moved (&mut self) -> () {
        self.has_moved = true;
    }
}

pub fn new (pos: Position, color: Color) -> Piece {
    Piece {
        name: Name::King,
        color,
        pos,
        data: Box::new(KingData {
            has_moved: false,
        })
    }
}
