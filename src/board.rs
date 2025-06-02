use std::ops::{Index, IndexMut};

use crate::piece::pawn::PawnData;
use crate::piece::{Color, Name, Piece, Position};
use crate::utils::{all_positions, validate_pos};

#[derive(Debug,Clone,Default)]
pub struct Board(pub [[Option<Piece>; 8]; 8]);

impl Index<usize> for Board {
    type Output = [Option<Piece>; 8];

    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.0[row]
    }
}

impl Board {
    pub fn get (&self, row: i8, col: i8) -> Option<&Piece> {
        if !validate_pos(row, col) { return None; }
        self[row as usize][col as usize].as_ref()
    }

    pub fn get_mut (&mut self, row: i8, col: i8) -> Option<&mut Piece> {
        if !validate_pos(row, col) { return None; }
        self[row as usize][col as usize].as_mut()
    }

    pub fn take (&mut self, row: i8, col: i8) -> Option<Piece> {
        if !validate_pos(row, col) { return None; }
        self[row as usize][col as usize].take()
    }

    pub fn set (&mut self, pos: Position, piece: Option<Piece>) -> () {
        if !validate_pos(pos.row as i8, pos.col as i8) { return; }
        self[pos.row][pos.col] = piece;
    }

    pub fn is_enemy_cell (&self, pos: Position, color: Color) -> bool {
        self.get(pos.row as i8, pos.col as i8).map_or(false, |p| p.color != color)
    }

    pub fn is_empty_cell (&self, pos: Position) -> bool {
        self.get(pos.row as i8, pos.col as i8).is_none()
    }

    pub fn get_king_pos (&self, color: Color) -> Option<Position> {
        all_positions()
            .find(|&pos| {
                self.get(pos.row as i8, pos.col as i8)
                    .map_or(false, |p| p.name == Name::King && p.color == color)
            })
    }

    pub fn clear_passants (&mut self, color: Color) -> () {
        for pos in all_positions() {
            if let Some(piece) = self.get_mut(pos.row as i8, pos.col as i8) {
                if piece.color != color || piece.name != Name::Pawn { continue; }
                if let Some(pawn_data) = piece.data.as_any_mut().downcast_mut::<PawnData>() {
                    pawn_data.passant_target = None;
                }
            }
        }
    }
}
