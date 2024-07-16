use std::fmt;
use crate::game_structures::*;

pub enum MoveType {
    Normal,
    Capture,
    Special,
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            &MoveType::Normal => "Normal",
            &MoveType::Capture => "Capture",
            &MoveType::Special => "Special",
        })
    }
}

pub struct Move {
    pub move_type: MoveType,
    pub from: u8,
    pub to: u8,
}

pub fn getboard_index(n: u8) -> String {
    let letters = ["A", "B", "C", "D", "E", "F", "G", "H"];
    let mut x = n;
    let mut index = 0;
    while x % 8 != 0 {
        x -= 1;
        index += 1;
    }
    return format!("{}{}", letters[index], 8 - (n / 8));
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Move from {} => {}", self.move_type ,getboard_index(self.from), getboard_index(self.to))
    }
}

pub fn get_moves(board: ChessBoard, piece: Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    match piece {
        Piece::P(side) => {
            if !side {
                for position in deserialize(board.get_board(piece)) {
                    moves.push(Move {
                        move_type: MoveType::Normal,
                        from: position as u8,
                        to: (position + 8) as u8,
                    });
                    moves.push(Move {
                        move_type: MoveType::Capture,
                        from: position as u8,
                        to: (position + 9) as u8
                    });
                    moves.push(Move {
                        move_type: MoveType::Capture,
                        from: position as u8,
                        to: (position + 7) as u8
                    });
                }
            } else {
                for position in deserialize(board.get_board(piece)) {
                    moves.push(Move {
                        move_type: MoveType::Normal,
                        from: position as u8,
                        to: (position - 8) as u8,
                    });
                    moves.push(Move {
                        move_type: MoveType::Capture,
                        from: position as u8,
                        to: (position - 9) as u8
                    });
                    moves.push(Move {
                        move_type: MoveType::Capture,
                        from: position as u8,
                        to: (position - 7) as u8
                    });
                }
            }
        }
        Piece::B(_) => unimplemented!(),
        Piece::K(_) => unimplemented!(),
        Piece::N(_) => unimplemented!(),
        Piece::Q(_) => unimplemented!(),
        Piece::R(_) => unimplemented!(),
        Piece::Multiple(_) => unimplemented!(),
    };
    moves
}
