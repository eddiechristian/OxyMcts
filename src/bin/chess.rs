use oxymcts::GameTrait;

use oxymcts::chess::chess::{Player, FEN_INITIAL_STATE,get_legal_moves, game_move_piece, get_game_result};

use std::fmt::{Display, Formatter, self};
use std::{collections::HashMap};

const WHITE_PAWN: char = '\u{2659}';
const WHITE_ROOK: char = '\u{2656}';
const WHITE_KNIGHT: char = '\u{2658}';
const WHITE_BISHOP: char = '\u{2657}';

const BLACK_PAWN: char = '\u{265F}';
const BLACK_ROOK: char = '\u{265C}';
const BLACK_KNIGHT: char = '\u{265E}';
const BLACK_BISHOP: char = '\u{265D}';
const BLACK_QUEEN: char = '\u{265B}';
const BLACK_KING: char = '\u{265A}';

const WHITE_QUEEN: char = '\u{2655}';
const WHITE_KING: char = '\u{2654}';

pub static TOP_BORDER:  &'static str =    "┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓";

pub static MIDDLE_BORDER:  &'static str = "┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫";

pub static BOTTOM_BORDER:  &'static str = "┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛";

#[derive(Debug, Clone, Default)]
struct ChessMCTS {
    turn: Player,
    fen_string: String,
}

impl ChessMCTS {
    fn new(state: &str) -> Self {
        ChessMCTS {
            turn: Player::White,
            fen_string: state.to_owned(),
        }
    }

    pub fn get_turn(&self) -> Player {
        self.turn.clone()
    }

    pub fn legal_moves(&self) -> Vec<String> {
        let mut legal_moves_vec = Vec::new();
        let (legal_moves ,_) = get_legal_moves(&self.fen_string);
        for  (from_spot, to_spots) in legal_moves.iter() {
            for to_spot in to_spots {
                let chess_move = format!("{}{}", from_spot, to_spot);
                legal_moves_vec.push(chess_move);
            }
        }
        legal_moves_vec
    }

}

impl GameTrait for ChessMCTS {
    type Player = Player;
    type Move = String;

    fn legals_moves(&self) -> Vec<Self::Move> {
        self.legal_moves()
    }

    fn player_turn(&self) -> Self::Player {
        self.get_turn()
    }

    fn hash(&self) -> u64 {
        0
    }

    fn is_final(&self) -> bool {
        self.legal_moves().is_empty()
        //todo test for 50 move rule
    }

    fn do_move(&mut self, m: &Self::Move) {
        let (fen_string, _, _) = game_move_piece(&self.fen_string, m);
        self.fen_string = fen_string;
    }

    fn get_winner(&self) -> Self::Player {
        todo!()
    }
}

impl Display for ChessMCTS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        println!("{}",self.fen_string);
        let placement_opt = self.fen_string.split(" ").next();
        if let Some(placement)= placement_opt {
            let mut row =8;
            write!(f, "\n{}\n" ,TOP_BORDER);
            
            for piece in placement.chars() {
                match piece {
                    'r' => { write!(f,"┃ {} ", BLACK_ROOK)?; },
                    'n' => {write!(f, "┃ {} ", BLACK_KNIGHT)?;},
                    'b' => {write!(f, "┃ {} ", BLACK_BISHOP)?;},
                    'q' => {write!(f, "┃ {} ", BLACK_QUEEN)?;},  
                    'k' => {write!(f, "┃ {} ", BLACK_KING)?;},  
                    'p' => {write!(f, "┃ {} ", BLACK_PAWN)?;},  
                    '/' => {
                        write!(f, "┃{}\n{}\n", row, MIDDLE_BORDER)?;
                        row -=1;
                    },
                    '8' => {write!(f, "┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃   ")?;}, 
                    '7' => {write!(f, "┃   ┃   ┃   ┃   ┃   ┃   ┃   ")?;}, 
                    '6' => {write!(f, "┃   ┃   ┃   ┃   ┃   ┃   ")?;}, 
                    '5' => {write!(f, "┃   ┃   ┃   ┃   ┃   ")?;}, 
                    '4' => {write!(f, "┃   ┃   ┃   ┃   ")?;}, 
                    '3' => {write!(f, "┃   ┃   ┃   ")?;}, 
                    '2' => {write!(f, "┃   ┃   ")?;}, 
                    '1' => {write!(f, "┃   ")?;},
                    'P' => { write!(f, "┃ {} ", WHITE_PAWN)?; },
                    'R' => { write!(f, "┃ {} ", WHITE_ROOK)?; },
                    'N' => { write!(f, "┃ {} ", WHITE_KNIGHT)?; },
                    'B' => { write!(f, "┃ {} ", WHITE_BISHOP)?; },
                    'Q' => { write!(f, "┃ {} ", WHITE_QUEEN)?; },
                    'K' => { write!(f, "┃ {} ", WHITE_KING)?; },
                    _ => {}
                }
            }
            write!(f, "┃{}\n{}\n" ,row, BOTTOM_BORDER);
            write!(f, "  a   b   c   d   e   f   g   h\n")
        }else {
            write!(f, "")
        }
       

        
        
    }
}

fn main() {
    let chess = ChessMCTS::new("8/2Q5/k7/6B1/1P6/p1NP4/P1P3BP/R3K1NR b KQ - 5 45");
    println!("{}", chess);
    let x = get_game_result("8/2Q5/k7/6B1/1P6/p1NP4/P1P3BP/R3K1NR b KQ - 5 45");
    println!("{:?}", x);
}