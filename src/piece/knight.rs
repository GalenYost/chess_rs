use crate::utils::step_moves;
use crate::piece::{Piece, PieceData, Position, Name, Color};
use crate::board::Board;

use std::any::Any;

static STEPS: &[(i8, i8)] = &[
    ( 2,  1), ( 2, -1),
    (-2,  1), (-2, -1),
    ( 1,  2), ( 1, -2),
    (-1,  2), (-1, -2),
];

#[derive(Clone,Debug)]
pub struct KnightData {}

impl PieceData for KnightData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        step_moves(board, pos, STEPS, color)
    }

    fn on_move (&mut self, _from: Position, _to: Position, _color: Color, _board: &mut Board) -> () {}
}

pub fn new (pos: Position, color: Color) -> Piece {
    Piece {
        name: Name::Knight,
        color,
        pos,
        data: Box::new(KnightData {})
    }
}
