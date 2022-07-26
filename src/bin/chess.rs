use oxymcts::{GameTrait, random_agent, mcts_uct_agent};

use oxymcts::chess::chess::{Player, FEN_INITIAL_STATE,get_legal_moves, game_move_piece, get_game_result, GameResult};
use oxymcts::chess::fen::FenRecord;

use tracing::{info, Level};
use num_traits::FloatConst;
use std::fmt::{Display, Formatter, self};
use std::io;
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
pub struct ChessMCTS {
    turn: u8, // 1= white , 2= black
    fen_string: String,
}

impl ChessMCTS {
    fn new(state: &str) -> Self {
        ChessMCTS {
            turn: 1,
            fen_string: state.to_owned(),
        }
    }

    pub fn get_turn(&self) -> u8 {
        self.turn
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
    pub fn play(&mut self, chess_move: String) {
        let (fen_string, _, _) = game_move_piece(&self.fen_string, &chess_move);
        self.fen_string = fen_string;
        let fen_record = FenRecord::from(&self.fen_string.to_owned());
        self.turn = match fen_record.player {
            'w' => 1,
            'b' => 2,
            _   => 0
        };
        if self.turn ==0 {
            panic!()
        }
    }

}

impl GameTrait for ChessMCTS {
    type Player = u8;
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
        get_game_result(&self.fen_string).is_some()
    }

    fn do_move(&mut self, m: &Self::Move) {
        self.play(m.clone())
    }

    fn get_winner(&self) -> Self::Player {
        match get_game_result(&self.fen_string) {
            Some(game_result) => {
                match game_result {
                    GameResult::BlackWins => 2,
                    GameResult::WhiteWins => 1,
                    _ => 0
                }
            },
            None => 0
        }
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
    let file_appender = tracing_appender::rolling::hourly("/Users/edwardchristian/source/rust_play/OxyMcts/logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
   tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format::format().pretty())
        .with_max_level(Level::TRACE)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
       .with_writer(non_blocking)
       .init();

    println!("Player 1: White (Random bot)");
    println!("Player 2: Black (MCTS)");
    let mut buffer = String::new();
    let mut chess = ChessMCTS::new(FEN_INITIAL_STATE);
    while !chess.is_final() {
        println!("Random turn: ");
        let move_random = dbg!(random_agent(&chess));
        chess.play(move_random);
        println!("{}", chess);
        //io::stdin().read_line(&mut buffer).unwrap();
        if !chess.is_final() {
            println!("Mcts turn: ");
            let move_mcts = dbg!(mcts_uct_agent(&chess, 10, f64::SQRT_2()));
            chess.play(move_mcts);
            println!("{}", chess);
            //io::stdin().read_line(&mut buffer).unwrap();
        }
    }

    // let chess = ChessMCTS::new("8/2Q5/k7/6B1/1P6/p1NP4/P1P3BP/R3K1NR b KQ - 5 45");
    // println!("{}", chess);
    // let x = get_game_result("8/2Q5/k7/6B1/1P6/p1NP4/P1P3BP/R3K1NR b KQ - 5 45");
    // println!("{:?}", x);
}