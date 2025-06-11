use crate::utils::sliding_moves;
use crate::piece::{Color, MoveMeta, Name, Piece, PieceData, Position};
use crate::board::Board;

use std::any::Any;

static DIRS: &[(i8, i8)] = &[
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];


#[derive(Clone,Debug)]
pub struct BishopData {}

impl PieceData for BishopData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        sliding_moves(board, pos, DIRS, Some(color))
    }

    fn on_move (&mut self, _from: Position, _to: Position, _color: Color, _board: &mut Board) -> Option<MoveMeta> {None}
}

pub fn new (pos: Position, color: Color) -> Piece {
    Piece {
        name: Name::Bishop,
        color,
        pos,
        data: Box::new(BishopData {})
    }
}
