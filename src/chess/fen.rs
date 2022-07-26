use std::cell::RefCell;
use std::fmt;

use super::chess::{Chess, Piece, PieceType, FEN_INITIAL_STATE, INIT, ChessState, Player};

pub(crate) struct FenRecord {
    pub piece_placement_data: String,
    pub player: char,
    pub castling: String,
    pub en_passant_target: String,
    pub halfmove_clock: u8,
    pub full_move_number: u16,
}

impl From<&FenRecord> for Chess {
    fn from(fen_record: &FenRecord) -> Self {
        let mut rows_of_pieces: Vec<&str> = fen_record
            .piece_placement_data
            .split("/")
            .collect::<Vec<&str>>();

        let mut state: [Option<Piece>; 64] = [INIT; 64];
        let mut index: usize = 0;
        for row in rows_of_pieces {
            for piece_char in row.chars() {
                match piece_char {
                    'p' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackPawn,
                        });
                        index += 1;
                    }
                    'r' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackRook,
                        });
                        index += 1;
                    }
                    'n' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackKnight,
                        });
                        index += 1;
                    }
                    'b' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackBishop,
                        });
                        index += 1;
                    }
                    'q' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackQueen,
                        });
                        index += 1;
                    }
                    'k' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::BlackKing,
                        });
                        index += 1;
                    }
                    'P' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhitePawn,
                        });
                        index += 1;
                    }
                    'R' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteRook,
                        });
                        index += 1;
                    }
                    'N' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteKnight,
                        });
                        index += 1;
                    }
                    'B' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteBishop,
                        });
                        index += 1;
                    }
                    'Q' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteQueen,
                        });
                        index += 1;
                    }
                    'K' => {
                        state[index] = Some(Piece {
                            piece_type: PieceType::WhiteKing,
                        });
                        index += 1;
                    }
                    num_empty_char => {
                        let num_empty = num_empty_char.to_string().parse::<u16>().unwrap();
                        for j in 0..num_empty {
                            state[index] = None;
                            index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Chess {
            state: ChessState {st: state},
            full_move_number: fen_record.full_move_number,
            castling: Some(fen_record.castling.clone()),
            halfmove_clock: fen_record.halfmove_clock,
            player: match fen_record.player {
                'w' => Player::White,
                _ => Player::Black,
            },
            en_passant_target:  Some(fen_record.en_passant_target.clone()),
            game_over: RefCell::new(None),
        }
    }
}

impl Default for FenRecord {
    fn default() -> Self {
        Self {
            piece_placement_data: FEN_INITIAL_STATE.to_owned(),
            player: 'w',
            castling: "KQkq".to_owned(),
            en_passant_target: "-".to_owned(),
            halfmove_clock: Default::default(),
            full_move_number: Default::default(),
        }
    }
}
impl From<&String> for FenRecord {
    fn from(fen_string: &String) -> Self {
        let fields: Vec<&str> = fen_string.split(" ").collect();
        FenRecord {
            piece_placement_data: fields[0].to_owned(),
            player: fields[1].chars().next().unwrap(),
            castling: fields[2].to_owned(),
            en_passant_target: fields[3].to_owned(),
            halfmove_clock: fields[4].parse::<u8>().unwrap(),
            full_move_number: fields[5].parse::<u16>().unwrap(),
        }
    }
}

impl fmt::Display for FenRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.piece_placement_data,
            self.player,
            self.castling,
            self.en_passant_target,
            self.halfmove_clock,
            self.full_move_number
        )
    }
}
