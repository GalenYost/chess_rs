use crate::board::Board;
use crate::piece::{Color, Move, Name, Position};
use crate::rules::filter_moves;

pub struct Game {
    pub board: Board,
    pub history: Vec<Move>,
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
        self.turn = match self.turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
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

        if !legal_moves.contains(&to) { return; }

        if piece.name != Name::King && piece.name != Name::Pawn {
            self.history.push(Move {
                piece_name: piece.name,
                piece_color: piece.color,
                from,
                to,
                capture: self.board.get(to.row as i8, to.col as i8).is_some(),
                promotion: None,
                castle: false,
            })
        }

        let mut p = match self.board.take(from.row as i8, from.col as i8) {
            Some(p) => p,
            None => return,
        };

        p.pos = to;
        p.on_move(to, &mut self.board);
        self.board.set(to, Some(p));

        self.switch_turn();
    }
}
