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

#[derive(Debug,PartialEq,Eq,Clone,Copy,Default)]
pub enum Color {
    #[default]
    White,
    Black
}

#[derive(Default,Debug,PartialEq,Eq,Clone,Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

#[derive(Debug,Clone,Copy)]
pub struct MoveMeta {
    pub piece_name: Name,
    pub piece_color: Color,
    pub from: Position,
    pub to: Position,
    pub capture: bool,
    pub promotion: Option<Name>,
    pub castle: bool,
}

#[derive(Debug,Clone)]
pub struct Piece {
    pub name: Name,
    pub color: Color,
    pub pos: Position,
    pub data: Box<dyn PieceData>,
}

pub trait PieceData: PieceDataClone + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn legal_moves(&self, pos: Position, color: Color, board: &Board) -> Vec<Position>;
    fn on_move(&mut self, from: Position, to: Position, color: Color, board: &mut Board) -> Option<MoveMeta>;
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

impl Piece {
    pub fn legal_moves (&self, board: &Board) -> Vec<Position> {
        self.data.legal_moves(self.pos, self.color, board)
    }

    pub fn on_move (&mut self, to: Position, board: &mut Board) -> Option<MoveMeta> {
        self.data.on_move(self.pos, to, self.color, board)
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

impl Color {
    pub fn opposite (&self) -> Color {
        match self {
            Color::White => Color::Black,
            _ => Color::White
        }
    }
}
