use crate::utils::sliding_moves;
use crate::piece::{Color, MoveMeta, Name, Piece, PieceData, Position};
use crate::board::Board;

use std::any::Any;

static DIRS: &[(i8, i8)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

#[derive(Clone,Debug)]
pub struct RookData {
    pub has_moved: bool,
}

impl PieceData for RookData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        sliding_moves(board, pos, DIRS, Some(color))
    }

    fn on_move (&mut self, _from: Position, _to: Position, _color: Color, _board: &mut Board) -> Option<MoveMeta> {
        self.moved();
        None
    }
}

impl RookData {
    pub fn moved (&mut self) -> () {
        self.has_moved = true;
    }
}

pub fn new (pos: Position, color: Color) -> Piece {
    Piece {
        name: Name::Rook,
        color,
        pos,
        data: Box::new(RookData {
            has_moved: false,
        })
    }
}
