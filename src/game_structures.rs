use std::fmt;
use std::ops::{BitOr, BitAnd};
use thiserror::Error;



#[derive(Error, Debug)]
pub enum GameErrors {
    #[error("Are you playing mind-chess? A chess board only has 64 squares")]
    OutOfBound,
    #[error("No BitBoard for chess piece")]
    NoPieceBoard
}

#[derive(PartialEq)]
pub enum Piece {
    K(bool), Q(bool), R(bool), B(bool), N(bool), P(bool), Multiple(Vec<Piece>)
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn get_side(side: &bool) -> &str {
            if *side {
                "Black"
            }
            else {
                "White"
            }
        }

        fn write_pieces(pieces: &Vec<Piece>) -> String {
            let mut result = String::from("");
            for piece in pieces {
                result += &format!("{}, ", &piece).to_string();
            }
            result
        }
        match self {
            Piece::K(side) => write!(f, "{} King", get_side(side)),
            Piece::Q(side) => write!(f, "{} Queen", get_side(side)),
            Piece::R(side) => write!(f," {} Rook", get_side(side)),
            Piece::B(side) => write!(f, "{} Bishop", get_side(side)),
            Piece::N(side) => write!(f, "{} Knight", get_side(side)),
            Piece::P(side) => write!(f, "{} Pawn", get_side(side)),
            Piece::Multiple(pcs) => write!(f, "{}", write_pieces(pcs))
        }
    }
}

pub struct BitBoard {
    pub board: u64,
    pub piece: Piece,
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} BitBoard: {:b}",self.piece,self.board)
    }

}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let pieces = vec![self.piece, other.piece];

        BitBoard {piece: Piece::Multiple(pieces), board: (self.board | other.board)}
    }

}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, other:Self) -> Self {
        let pieces = vec![self.piece, other.piece];

        BitBoard {piece: Piece::Multiple(pieces), board: (self.board & other.board)}
    }
}

pub enum Board {
    Standard
}

impl Board {
    pub fn init(&self) -> Result<ChessBoard, GameErrors> {
        let white_pawns = serialize(Piece::P(false), vec![8, 9, 10, 11, 12, 13, 14, 15])?;
        let white_rooks = serialize(Piece::R(false), vec![0, 7])?;
        let white_knights = serialize(Piece::N(false), vec![1, 6])?;
        let white_bishops = serialize(Piece::B(false), vec![2, 5])?;
        let white_queens = serialize(Piece::Q(false), vec![3])?;
        let white_kings = serialize(Piece::K(false), vec![4])?;
       
        let black_pawns = serialize(Piece::P(true), vec![48, 49, 50, 51, 52, 53, 54, 55])?;
        let black_rooks = serialize(Piece::R(true), vec![56, 63])?;
        let black_knights = serialize(Piece::N(true), vec![57, 62])?;
        let black_bishops = serialize(Piece::B(true), vec![58, 61])?;
        let black_queens = serialize(Piece::Q(true), vec![59])?;
        let black_kings = serialize(Piece::K(true), vec![60])?;
        Ok(ChessBoard {all_boards: [
            white_pawns, white_rooks, white_knights, white_bishops, white_queens, white_kings,
            black_pawns, black_rooks, black_knights, black_bishops, black_queens, black_kings
        ]
        })
    }
}

pub struct ChessBoard {
    all_boards: [BitBoard; 12] 
}

impl ChessBoard {
    pub fn get_board(&self, piece: Piece) -> BitBoard {
        fn get_offset(side: bool) -> usize {
            if side {
                6
            } else {
                0
            }
        }
        let req_board = match piece {
            Piece::P(side) => &self.all_boards[get_offset(side)],
            Piece::R(side) => &self.all_boards[get_offset(side) + 1],
            Piece::N(side) => &self.all_boards[get_offset(side) + 2],
            Piece::B(side) => &self.all_boards[get_offset(side) + 3],
            Piece::Q(side) => &self.all_boards[get_offset(side) + 4],
            Piece::K(side) => &self.all_boards[get_offset(side) + 5],
            Piece::Multiple(_) => panic!("Multiple Piece BitBoard Unaccessible"), 
            }.board;

        BitBoard{board:req_board, piece}
    }
}

pub fn serialize(board_piece: Piece, positions: Vec<usize>) -> Result<BitBoard, GameErrors> {

    let mut bit_values: [bool; 64] = [false; 64];
    for p in positions {
        if p > 63 {
            return Err(GameErrors::OutOfBound)
        } else {
            bit_values[63 - p] = true;
        }
    }

    let mut result_string: String = String::new();
    for bit in bit_values {
        if bit {
            result_string.push('1');
        } else {
            result_string.push('0');
        }
    }
    Ok(BitBoard {
        board: u64::from_str_radix(&result_string, 2).unwrap(),
        piece: board_piece
    })
}

pub fn deserialize(board: BitBoard) -> (Piece, Vec<usize>) {
    let mut positions: Vec<usize> = vec![]; 
    let string_board = format!("{:b}", board.board);


    for (pos, bit) in string_board.chars().enumerate() {
        if bit == '1' {
            positions.push(pos + 1);
        }
    }

    (board.piece, positions)
}
