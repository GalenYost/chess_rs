pub mod pawn; pub mod rook;
pub mod king; pub mod queen;
pub mod knight; pub mod bishop;

use std::fmt::Debug;
use std::any::Any;

use crate::board::Board;
use crate::utils::validate_pos;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Name {
    Pawn, King, Queen, Rook, Knight, Bishop
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Color {
    White,
    Black
}

#[derive(Default,Debug,PartialEq,Eq,Clone,Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

#[derive(Debug,Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub capture: bool,
    pub promotion: Option<Piece>,
    pub castle: bool,
}

#[derive(Debug,Clone)]
pub struct Piece {
    pub name: Name,
    pub color: Color,
    pub data: Box<dyn PieceData>,
}

pub trait PieceData: PieceDataClone + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn legal_moves(&self, pos: Position, color: Color, board: &Board) -> Vec<Position>;
    fn on_move(&mut self, from: Position, to: Position, color: Color, board: &mut Board);
}

pub trait PieceDataClone {
    fn box_clone(&self) -> Box<dyn PieceData>;
}

impl<T> PieceDataClone for T
where
    T: PieceData + Clone + 'static,
{
    fn box_clone(&self) -> Box<dyn PieceData> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PieceData> {
    fn clone(&self) -> Box<dyn PieceData> {
        self.box_clone()
    }
}

impl Position {
    pub fn shifted (&self, dx: i8, dy: i8) -> Option<Position> {
        let ri = self.row as i8 + dy;
        let ci = self.col as i8 + dx;

        if validate_pos(ri, ci) {
            Some(Position {
                row: ri as usize,
                col: ci as usize,
            })
        } else {
            None
        }
    }
}
