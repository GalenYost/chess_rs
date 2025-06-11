use crate::board::Board;
use crate::piece::{Color, MoveMeta, Position};
use crate::rules::filter_moves;

pub struct Game {
    pub board: Board,
    pub history: Vec<MoveMeta>,
    turn: Color,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            turn: Color::default(),
            history: Vec::new(),
        }
    }
}

impl Game {
    fn switch_turn (&mut self) -> () {
        self.turn = self.turn.opposite();
    }

    pub fn get_turn (&self) -> Color {
        self.turn
    }

    pub fn apply_move (&mut self, from: Position, to: Position) -> () {
        let piece = match self.board.get(from.row as i8, from.col as i8) {
            Some(p) => p,
            None => return,
        };

        if self.turn != piece.color { return; }

        let mut legal_moves = piece.legal_moves(&self.board);
        filter_moves(&self.board, &mut legal_moves, from, piece.color);

        if !legal_moves.contains(&to) {
            println!("Log: illegal move");
            println!("Info: from - {:?}, to - {:?}, piece - {:?}", from, to, piece);
            return;
        }

        let mut p = match self.board.take(from.row as i8, from.col as i8) {
            Some(p) => p,
            None => return,
        };

        p.pos = to;
        if let Some(move_meta) = p.on_move(to, &mut self.board) {
            self.history.push(move_meta)
        } else {
            self.history.push(MoveMeta {
                piece_name: p.name,
                piece_color: p.color,
                from,
                to,
                capture: self.board.get(to.row as i8, to.col as i8).is_some(),
                promotion: None,
                castle: false
            })
        }
        self.board.set(to.row as i8, to.col as i8, Some(p));

        self.switch_turn();
    }
}
