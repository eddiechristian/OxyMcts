use core::num;
use std::{collections::HashMap, convert::{From, TryInto}, fmt::format, cell::RefCell};


use serde::{Deserialize, Serialize};

use crate::chess::{ king::get_king_unvalidated_moves};

use super::{
    chess_errors, chess_notation, fen::{FenRecord, self}, knight::get_knight_unvalidated_moves,
    pawn::get_pawn_unvalidated_moves, pawn::move_pawn_vertical , pawn::move_pawn_diagonal,
    queen::get_queen_unvalidated_moves, rook::get_rook_unvalidated_moves, bishop::get_bishop_unvalidated_moves
};

pub(crate) const INIT: Option<Piece> = None;

pub const FEN_INITIAL_STATE: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";

pub fn get_game_result(state: &str)-> Option<GameResult> {
    let mut chess = Chess::from(&FenRecord::from(&state.to_owned()));
    let fen_record: FenRecord = FenRecord::from(&chess);
    let valid_moves = chess.get_legal_moves().unwrap();
    if valid_moves.is_empty() {
        //determine if opoosing player can attack king, if not stalemate
        //replace player in current fen_string to determine if it can attack king.
        let mut fen_record = FenRecord::from(&state.to_owned());
        fen_record.player = {
            match fen_record.player {
                'w'=> 'b',
                'b'=> 'w',
                _ => panic!()
            }
        };
        let mut chess_opposing = Chess::from(&fen_record);
        let opposing_valid_moves = chess_opposing.get_legal_moves().unwrap();
        for (cloned_from_spot, cloned_to_spots) in opposing_valid_moves {
            for cloned_to_spot in cloned_to_spots {
                let cloned_move = format!("{}{}",cloned_from_spot,cloned_to_spot );
                //only care about moves that attack opposing king
                let cloned_to_spot_index = chess_notation::notation_to_index(&cloned_to_spot).unwrap();
                if let Some(piece) = &chess_opposing.state.st[cloned_to_spot_index] {
                    if piece.get_player() != chess_opposing.player  {
                        if piece.piece_type == PieceType::BlackKing || piece.piece_type == PieceType::WhiteKing {
                            if chess_opposing.is_move_valid(&cloned_from_spot, &cloned_to_spot , None).is_ok(){ 
                                let game_result= match chess_opposing.player {
                                    Player::Black => GameResult::BlackWins,
                                    Player::White => GameResult::WhiteWins,
                                };
                                return Some(game_result)
                            }
                            
                        }
                    }
                }
            }
        }
        Some(GameResult::Stalemate)
    }else {
        //determin if 50 move rule is broken, else return none
        if fen_record.halfmove_clock > 50 {
            return Some(GameResult::Stalemate)
        }else {
            return None
        }
        
    }
}

///
/// 
pub fn get_legal_moves(
    state: &str,
) -> (HashMap<String,Vec<String>>, WebGame) {
    let mut chess = Chess::from(&FenRecord::from(&state.to_owned()));
    let fen_record: FenRecord = FenRecord::from(&chess);
    let valid_moves = chess.get_legal_moves().unwrap();
    println!("fen_record {}", fen_record);
    let web_game:WebGame = WebGame::from(&chess);
    (valid_moves,web_game)
}

pub fn game_move_piece( state: &str ,chess_move: &str) -> (String, WebGame, HashMap<String,Vec<String>>) {
    let mut chess = Chess::from(&FenRecord::from(&state.to_owned()));
    chess.move_piece(&chess_move);
    let valid_moves = chess.get_legal_moves().unwrap();
    return (FenRecord::from(&chess).to_string(), WebGame::from(&chess), valid_moves);
}


#[derive(Debug,Clone, PartialEq, Eq)]
pub enum Player {
    White,
    Black
}
impl Default for Player {
    fn default() -> Self { Player::White }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MoveType {
    Enpassant(usize),
    Castling,
    Regular,
    Promotion(PieceType)
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PieceType {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}
#[derive(Clone, Debug)]
pub(crate) struct Piece {
    pub piece_type: PieceType,
}

impl Piece {
    pub fn get_player(&self) -> Player {
        match self.piece_type {
            PieceType::WhitePawn
            | PieceType::WhiteRook
            | PieceType::WhiteKnight
            | PieceType::WhiteBishop
            | PieceType::WhiteQueen
            | PieceType::WhiteKing => {
                return Player::White;
            }
            _ => {
                return Player::Black;
            }
        }
    }

    fn move_horizontal(&self, to_spot: &str, state: &[Option<Piece>; 64], delta_x: i8, promotion: Option<&str>,castling: Option<String> ) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        match self.piece_type {
            PieceType::BlackPawn | PieceType::WhitePawn | 
            PieceType::BlackBishop | PieceType::WhiteBishop |
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
            },
            PieceType::BlackQueen | PieceType::WhiteQueen |
            PieceType::BlackRook | PieceType::WhiteRook
            => {
                if promotion.is_some() {
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
                }
                Ok((to_spot.to_string(),MoveType::Regular))
            }
            PieceType::BlackKing => {
                if delta_x == 2 {
                    if castling.unwrap().contains("q") {
                        Ok((to_spot.to_string(),MoveType::Castling))
                    } else {
                        let msg = format!("{}",to_spot);
                        return Err(chess_errors::ChessErrors::InvalidMove(msg));
                    }
                } else if delta_x == -2 {
                    if castling.unwrap().contains("k") {
                        Ok((to_spot.to_string(),MoveType::Castling))
                    } else {
                        let msg = format!("{}",to_spot);
                        return Err(chess_errors::ChessErrors::InvalidMove(msg));
                    }
                }
                else {
                    Ok((to_spot.to_string(),MoveType::Regular))
                }
                
            }
            PieceType::WhiteKing => {
                if delta_x == 2 {
                    if castling.unwrap().contains("Q") {
                        Ok((to_spot.to_string(),MoveType::Castling))
                    } else {
                        let msg = format!("{}",to_spot);
                        return Err(chess_errors::ChessErrors::InvalidMove(msg));
                    }
                } else if delta_x == -2 {
                    if castling.unwrap().contains("K") {
                        Ok((to_spot.to_string(),MoveType::Castling))
                    } else {
                        let msg = format!("{}",to_spot);
                        return Err(chess_errors::ChessErrors::InvalidMove(msg));
                    }
                }
                else {
                    Ok((to_spot.to_string(),MoveType::Castling))
                }

            }
        }
    }
    fn move_vertical(&self, to_spot: &str,from_spot: &str, state: &[Option<Piece>; 64], delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        match self.piece_type {
           PieceType::BlackBishop | PieceType::WhiteBishop |
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
            },
            PieceType::BlackQueen | PieceType::WhiteQueen |
            PieceType::BlackRook | PieceType::WhiteRook |
            PieceType::BlackKing | PieceType::WhiteKing
            => {
                if promotion.is_some() {
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
                }
                Ok((to_spot.to_string(),MoveType::Regular))
            }
            PieceType::BlackPawn | PieceType::WhitePawn => {
                move_pawn_vertical(self, to_spot, from_spot, state, delta_y, promotion)
            }

        }
    }
    fn move_diagonal(&self, to_spot: &str, state: &[Option<Piece>; 64], delta_y: i8, promotion: Option<&str>,enpassant_target: Option<String>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        match self.piece_type {
            PieceType::BlackKnight | PieceType::WhiteKnight |
            PieceType::BlackRook | PieceType::WhiteRook => {
                 let msg = format!("{}",to_spot);
                 return Err(chess_errors::ChessErrors::InvalidMove(msg));
             },
             PieceType::BlackQueen | PieceType::WhiteQueen |
             PieceType::BlackKing | PieceType::WhiteKing | 
             PieceType::BlackBishop | PieceType::WhiteBishop
             => {
                 if promotion.is_some() {
                     let msg = format!("{}",to_spot);
                     return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
                 }
                 Ok((to_spot.to_string(),MoveType::Regular))
             }
             PieceType::BlackPawn | PieceType::WhitePawn => {
                move_pawn_diagonal(self, to_spot, state, delta_y, promotion, enpassant_target)
             }
 
         }
    }
    fn move_knight(&self, to_spot: &str, state: &[Option<Piece>; 64], promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        match self.piece_type {
            PieceType::BlackRook | PieceType::WhiteRook |
            PieceType::BlackQueen | PieceType::WhiteQueen |
            PieceType::BlackKing | PieceType::WhiteKing | 
            PieceType::BlackBishop | PieceType::WhiteBishop |
            PieceType::BlackPawn | PieceType::WhitePawn => {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
            }
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                Ok(to_spot.to_string())
            }
    }
}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebGame {
    state: [[char;8];8],
}

impl From<&Chess> for WebGame {
    fn from(chess: &Chess) -> Self {
        let mut state = [['.';8];8];
        for (index, piece_opt) in chess.state.st.iter().enumerate() {
            let piece_type =
            match piece_opt {
                Some(piece) =>{
                    match piece.piece_type {
                        PieceType::BlackBishop => 'b',
                        PieceType::BlackKing =>   'k',
                        PieceType::BlackKnight => 'n',
                        PieceType::BlackPawn =>   'p',
                        PieceType::BlackQueen =>  'q',
                        PieceType::BlackRook =>   'r',
                        PieceType::WhiteBishop => 'B',
                        PieceType::WhiteKing =>   'K',
                        PieceType::WhiteKnight => 'N',
                        PieceType::WhitePawn =>   'P',
                        PieceType::WhiteQueen =>  'Q',
                        PieceType::WhiteRook =>   'R',
                    }
                }
                None => '.',
            };
            let row = index /8;
            let col = index % 8;
            state[row][col]=piece_type;
        }
        WebGame {
            state,
        }
    }

}
#[derive(Debug, Clone)]
pub(crate) struct ChessState {
    pub st: [Option<Piece>; 64],
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Stalemate,
}

#[derive(Debug, Clone)]
pub(crate) struct Chess {
    pub state: ChessState,
    pub player: Player,
    pub castling: Option<String>,
    pub halfmove_clock: u8,
    pub full_move_number: u16,
    pub en_passant_target: Option<String>,
    pub game_over: RefCell<Option<GameResult>>,
}

impl From<&Chess> for FenRecord {
    fn from(chess: &Chess) -> Self {
        let mut piece_placement_data = "".to_owned();
        for (index, piece_opt) in chess.state.st.iter().enumerate() {
            if index != 0 && index % 8 == 0 {
                piece_placement_data.push_str("/");
            }
           
            let piece_type = {
                match piece_opt {
                    Some(piece) => {
                        match piece.piece_type {
                        PieceType::BlackBishop => "b",
                        PieceType::BlackKing =>   "k",
                        PieceType::BlackKnight => "n",
                        PieceType::BlackPawn =>   "p",
                        PieceType::BlackQueen =>  "q",
                        PieceType::BlackRook =>   "r",
                        PieceType::WhiteBishop => "B",
                        PieceType::WhiteKing =>   "K",
                        PieceType::WhiteKnight => "N",
                        PieceType::WhitePawn =>   "P",
                        PieceType::WhiteQueen =>  "Q",
                        PieceType::WhiteRook =>   "R",
                        }
                    },
                    None => "."
                }
               
        };
        piece_placement_data.push_str(piece_type);
    }
        
        piece_placement_data = piece_placement_data.replace("........", "8");
        piece_placement_data = piece_placement_data.replace(".......", "7");
        piece_placement_data = piece_placement_data.replace("......", "6");
        piece_placement_data = piece_placement_data.replace(".....", "5");
        piece_placement_data = piece_placement_data.replace("....", "4");
        piece_placement_data = piece_placement_data.replace("...", "3");
        piece_placement_data = piece_placement_data.replace("..", "2");
        piece_placement_data = piece_placement_data.replace(".", "1");
        let player = match chess.player {
            Player::Black => 'b',
            Player::White => 'w'
        };
        let en_passant_target = match &chess.en_passant_target {
            Some(target) => target.clone(),
            None => "-".to_owned()
        };
        let castling = match &chess.castling {
            Some(castling) => castling.clone(),
            None => "-".to_owned()
        };
        FenRecord {
            piece_placement_data: piece_placement_data,
            player: player,
            castling: castling,
            en_passant_target: en_passant_target,
            halfmove_clock: chess.halfmove_clock,
            full_move_number: chess.full_move_number,
        }
    }
}

impl Chess {
    pub fn getGameResult (&self) -> &RefCell<Option<GameResult>>{
        return &self.game_over;
    }
    pub fn check_for_check (cloned_game: &mut Chess ,chess_move: &str)->Result<(), chess_errors::ChessErrors> {
        let the_move = chess_move.to_lowercase();
        let from_spot = &the_move[0..2];
        let to_spot = &the_move[2..4];
        let mut en_passant_set_this_move =false;
        
        if let Ok((from, to)) = chess_notation::convert_move_notation_to_indexes(from_spot,to_spot) {
            let promotion = if the_move.len() > 4 {
                Some(&the_move[3..])
            } else {
                None
            };
            let move_type = cloned_game.is_move_valid(from_spot, to_spot, promotion)?;
            // if this is a king adjust castling.
            if cloned_game.state.st[from].as_ref().unwrap().piece_type  == PieceType::WhiteKing {
                let new_castling = cloned_game.castling.as_mut().unwrap().replace("KQ", "");
                cloned_game.castling = Some(new_castling);
            } else if cloned_game.state.st[from].as_ref().unwrap().piece_type  == PieceType::BlackKing {
                let new_castling = cloned_game.castling.as_mut().unwrap().replace("kq", "");
                cloned_game.castling = Some(new_castling);
            }
            //if this is a rook adjust castling
            if cloned_game.state.st[from].as_ref().unwrap().piece_type  == PieceType::WhiteRook {
                if from_spot == "a1"{
                    let new_castling = cloned_game.castling.as_mut().unwrap().replace("Q", "");
                    cloned_game.castling = Some(new_castling);
                } else if from_spot == "h1" {
                    let new_castling = cloned_game.castling.as_mut().unwrap().replace("K", "");
                    cloned_game.castling = Some(new_castling);
                }
               
            } else if cloned_game.state.st[from].as_ref().unwrap().piece_type  == PieceType::BlackRook {
                if from_spot == "a8"{
                    let new_castling = cloned_game.castling.as_mut().unwrap().replace("q", "");
                    cloned_game.castling = Some(new_castling);
                } else if from_spot == "h8" {
                    let new_castling = cloned_game.castling.as_mut().unwrap().replace("k", "");
                    cloned_game.castling = Some(new_castling);
                }
            }
            //if this is a pawn and move was two squares adjust enpassant.
            let (from_point , to_point) = chess_notation::convert_move_notation_to_xy(from_spot, to_spot)?;
            let delta_y = from_point.y as i8  - to_point.y as i8 ;
            if delta_y.abs() == 2 {
                if cloned_game.state.st[from].as_ref().unwrap().piece_type  == PieceType::WhitePawn {
                    let enpassant_spot = chess_notation::index_to_spot(to + 8);
                    cloned_game.en_passant_target = Some(enpassant_spot);
                    en_passant_set_this_move = true
                }else if cloned_game.state.st[from].as_ref().unwrap().piece_type  == PieceType::BlackPawn {
                    let enpassant_spot = chess_notation::index_to_spot(to - 8);
                    cloned_game.en_passant_target = Some(enpassant_spot);
                    en_passant_set_this_move = true
                }
            }
            //here down we are mutating state.... we should mutate a cloned state and then test for check with cloned state
            let to_piece = std::mem::replace(&mut cloned_game.state.st[from], None);
            if let MoveType::Promotion(piece_type) =  move_type.clone() {
                let piece = Piece {
                    piece_type,
                };
                cloned_game.state.st[to] = Some(piece);
                cloned_game.player = {
                    match cloned_game.player {
                        Player::Black => Player::White,
                        Player::White => Player::Black,
                    }
                };
                return Ok(());
            }
            if move_type == MoveType::Castling {
                if to_spot == "g8" {
                    //black king side
                    let to_piece = std::mem::replace(&mut cloned_game.state.st[chess_notation::notation_to_index("h8").unwrap()], None);
                    cloned_game.state.st[chess_notation::notation_to_index("f8").unwrap()] = to_piece;
                } else if to_spot == "c8" {
                    //black queen side
                    let to_piece = std::mem::replace(&mut cloned_game.state.st[chess_notation::notation_to_index("a8").unwrap()], None);
                    cloned_game.state.st[chess_notation::notation_to_index("d8").unwrap()] = to_piece;
                }else if to_spot == "g1" {
                    //white king side
                    let to_piece = std::mem::replace(&mut cloned_game.state.st[chess_notation::notation_to_index("h1").unwrap()], None);
                    cloned_game.state.st[chess_notation::notation_to_index("f1").unwrap()] = to_piece;
                }else if to_spot == "c1" {
                    //white queen side
                    let to_piece = std::mem::replace(&mut cloned_game.state.st[chess_notation::notation_to_index("a1").unwrap()], None);
                    cloned_game.state.st[chess_notation::notation_to_index("d1").unwrap()] = to_piece;
                }
            }
            if let MoveType::Enpassant(index) =  move_type.clone() { 
                std::mem::replace(&mut cloned_game.state.st[index], None);
            }
            cloned_game.state.st[to] = to_piece;
            cloned_game.player = {
                match cloned_game.player {
                    Player::Black => Player::White,
                    Player::White => Player::Black,
                }
            };
           //clear enpassant
           if !en_passant_set_this_move {
            cloned_game.en_passant_target = None;
           }
           
        } else {
            let msg = format!("Invalid notation");
            return Err(chess_errors::ChessErrors::InvalidNotation(msg));
        }
    
        Ok(())
    }

    pub fn move_piece (&mut self ,chess_move: &str)->Result<(), chess_errors::ChessErrors> {
        let the_move = chess_move.to_lowercase();
        let from_spot = &the_move[0..2];
        let to_spot = &the_move[2..4];
         let mut en_passant_set_this_move =false;

        if let Ok((from, to)) = chess_notation::convert_move_notation_to_indexes(from_spot,to_spot) {
            let promotion = if the_move.len() > 4 {
                Some(&the_move[3..])
            } else {
                None
            };
            let move_type = self.is_move_valid(from_spot, to_spot, promotion)?;
            // if this is a king adjust castling.
            if self.state.st[from].as_ref().unwrap().piece_type  == PieceType::WhiteKing {
                let new_castling = self.castling.as_mut().unwrap().replace("KQ", "");
                self.castling = Some(new_castling);
            } else if self.state.st[from].as_ref().unwrap().piece_type  == PieceType::BlackKing {
                let new_castling = self.castling.as_mut().unwrap().replace("kq", "");
                self.castling = Some(new_castling);
            }
            //if this is a rook adjust castling
            if self.state.st[from].as_ref().unwrap().piece_type  == PieceType::WhiteRook {
                if from_spot == "a1"{
                    let new_castling = self.castling.as_mut().unwrap().replace("Q", "");
                    self.castling = Some(new_castling);
                } else if from_spot == "h1" {
                    let new_castling = self.castling.as_mut().unwrap().replace("K", "");
                    self.castling = Some(new_castling);
                }
               
            } else if self.state.st[from].as_ref().unwrap().piece_type  == PieceType::BlackRook {
                if from_spot == "a8"{
                    let new_castling = self.castling.as_mut().unwrap().replace("q", "");
                    self.castling = Some(new_castling);
                } else if from_spot == "h8" {
                    let new_castling = self.castling.as_mut().unwrap().replace("k", "");
                    self.castling = Some(new_castling);
                }
            }
            //if this is a pawn and move was two squares adjust enpassant.
            let (from_point , to_point) = chess_notation::convert_move_notation_to_xy(from_spot, to_spot)?;
            let delta_y = from_point.y as i8  - to_point.y as i8 ;
            if delta_y.abs() == 2 {
                if self.state.st[from].as_ref().unwrap().piece_type  == PieceType::WhitePawn {
                    let enpassant_spot = chess_notation::index_to_spot(to + 8);
                    self.en_passant_target = Some(enpassant_spot);
                    en_passant_set_this_move = true
                }else if self.state.st[from].as_ref().unwrap().piece_type  == PieceType::BlackPawn {
                    let enpassant_spot = chess_notation::index_to_spot(to - 8);
                    self.en_passant_target = Some(enpassant_spot);
                    en_passant_set_this_move = true
                }
            }
            //here down we are mutating state.
            let to_piece = std::mem::replace(&mut self.state.st[from], None);
            if let Some(to_piece) =&self.state.st[to] {
                self.halfmove_clock = 0;
            }
            if let MoveType::Promotion(piece_type) =  move_type.clone() {
                let piece = Piece {
                    piece_type,
                };
                self.state.st[to] = Some(piece);
                self.player = {
                    match self.player {
                        Player::Black => Player::White,
                        Player::White => Player::Black,
                    }
                };
                return Ok(());
            }
            if move_type == MoveType::Castling {
                if to_spot == "g8" {
                    //black king side
                    let to_piece = std::mem::replace(&mut self.state.st[chess_notation::notation_to_index("h8").unwrap()], None);
                    self.state.st[chess_notation::notation_to_index("f8").unwrap()] = to_piece;
                } else if to_spot == "c8" {
                    //black queen side
                    let to_piece = std::mem::replace(&mut self.state.st[chess_notation::notation_to_index("a8").unwrap()], None);
                    self.state.st[chess_notation::notation_to_index("d8").unwrap()] = to_piece;
                }else if to_spot == "g1" {
                    //white king side
                    let to_piece = std::mem::replace(&mut self.state.st[chess_notation::notation_to_index("h1").unwrap()], None);
                    self.state.st[chess_notation::notation_to_index("f1").unwrap()] = to_piece;
                }else if to_spot == "c1" {
                    //white queen side
                    let to_piece = std::mem::replace(&mut self.state.st[chess_notation::notation_to_index("a1").unwrap()], None);
                    self.state.st[chess_notation::notation_to_index("d1").unwrap()] = to_piece;
                }
            }
            if let MoveType::Enpassant(index) =  move_type.clone() { 
                std::mem::replace(&mut self.state.st[index], None);
            }
            self.state.st[to] = to_piece;
            self.player = {
                match self.player {
                    Player::Black => Player::White,
                    Player::White => Player::Black,
                }
            };
           //clear enpassant
           if !en_passant_set_this_move {
            self.en_passant_target = None;
           }
           self.full_move_number += 1;
           self.halfmove_clock += 1;
           
        } else {
            let msg = format!("Invalid notation");
            return Err(chess_errors::ChessErrors::InvalidNotation(msg));
        }
    
        Ok(())
    }

    pub fn get_legal_moves(&self) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors>{
        
        let mut legal_moves_map = HashMap::new();
        let mut unvalidated_moves = self.get_unvalidated_moves();
       for (from_spot, to_spots) in unvalidated_moves.unwrap() {
            let mut legal_moves_vec = Vec::new();
            for to_spot in to_spots {
                let mut results_in_check = false;
                if self.is_move_valid(&from_spot, &to_spot , None).is_ok(){
                    let mut cloned_game = self.clone();
                    let this_move = format!("{}{}",from_spot,to_spot );
                    Chess::check_for_check(&mut cloned_game, &this_move);
                    let mut cloned_unvalidated_moves = cloned_game.get_unvalidated_moves();
                    for (cloned_from_spot, cloned_to_spots) in cloned_unvalidated_moves.unwrap() {
                        for cloned_to_spot in cloned_to_spots {
                            let cloned_move = format!("{}{}",cloned_from_spot,cloned_to_spot );
                            //only care about moves that attack opposing king
                            let cloned_to_spot_index = chess_notation::notation_to_index(&cloned_to_spot)?;
                            if let Some(piece) = &cloned_game.state.st[cloned_to_spot_index] {
                                if piece.get_player() != cloned_game.player  {
                                    if piece.piece_type == PieceType::BlackKing || piece.piece_type == PieceType::WhiteKing {
                                        if cloned_game.is_move_valid(&cloned_from_spot, &cloned_to_spot , None).is_ok(){ 
                                            results_in_check = true;
                                        }
                                        
                                    }
                                }
                            }
                        }
                    }
                    if !results_in_check {
                        legal_moves_vec.push(format!("{}", to_spot));
                    }
                }
            }
            if legal_moves_vec.len() > 0 {
                legal_moves_map.insert(from_spot, legal_moves_vec);
            }
            
       }
        Ok(legal_moves_map)
    }

    pub fn get_unvalidated_moves(
        &self,
    ) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    
        let mut unvalidated_moves = HashMap::new();
        for (index, piece_opt) in self.state.st.iter().enumerate() {
            if let Some(piece) = piece_opt {
                if piece.get_player() == self.player {
                match piece.piece_type {
                    PieceType::WhitePawn | PieceType::BlackPawn => {
                        let mut unvalidated_moves_pawn = get_pawn_unvalidated_moves(
                            piece,
                            &self.state.st,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_pawn.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                        //check for promotion
                        let row = index/8;
                        let col = index % 8;
                        if row == 1 &&  piece.piece_type == PieceType::WhitePawn{
                            let mut unvalidated_moves_vec = Vec::new();
                            let spot = chess_notation::index_to_spot(index);

                            let to_spot_queen = format!("{}8q",spot.chars().nth(0).unwrap());
                            let to_spot_rook = format!("{}8r",spot.chars().nth(0).unwrap());
                            let to_spot_knight = format!("{}8n",spot.chars().nth(0).unwrap());
                            let to_spot_bishop = format!("{}8b",spot.chars().nth(0).unwrap());
                            unvalidated_moves_vec.push(to_spot_queen);
                            unvalidated_moves_vec.push(to_spot_rook);
                            unvalidated_moves_vec.push(to_spot_knight);
                            unvalidated_moves_vec.push(to_spot_bishop);
                            
                            if col ==0 {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}8q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}8r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}8n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}8b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                            }else if col == 7 {
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}8q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}8r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}8n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}8b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            }else {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}8q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}8r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}8n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}8b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}8q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}8r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}8n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}8b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            }
                            unvalidated_moves
                            .entry(spot)
                            .or_insert_with(|| Vec::new())
                            .append(&mut unvalidated_moves_vec);
                        } else if row == 6 &&  piece.piece_type == PieceType::BlackPawn{
                            let mut unvalidated_moves_vec = Vec::new();

                            let spot = chess_notation::index_to_spot(index);
                            let to_spot_queen = format!("{}1q",spot.chars().nth(0).unwrap());
                            let to_spot_rook = format!("{}1r",spot.chars().nth(0).unwrap());
                            let to_spot_knight = format!("{}1n",spot.chars().nth(0).unwrap());
                            let to_spot_bishop = format!("{}1b",spot.chars().nth(0).unwrap());
                            unvalidated_moves_vec.push(to_spot_queen);
                            unvalidated_moves_vec.push(to_spot_rook);
                            unvalidated_moves_vec.push(to_spot_knight);
                            unvalidated_moves_vec.push(to_spot_bishop);
                            if col ==0 {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}1q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}1r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}1n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}1b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                            }else if col == 7 {
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}1q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}1r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}1n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}1b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            } else {
                                let right_spot= chess_notation::index_to_spot(index+1); 
                                let right_to_spot_queen = format!("{}1q",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_rook = format!("{}1r",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_knight = format!("{}1n",right_spot.chars().nth(0).unwrap());
                                let right_to_spot_bishop = format!("{}1b",right_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(right_to_spot_queen);
                                unvalidated_moves_vec.push(right_to_spot_rook);
                                unvalidated_moves_vec.push(right_to_spot_knight);
                                unvalidated_moves_vec.push(right_to_spot_bishop);
                                let left_spot= chess_notation::index_to_spot(index-1); 
                                let left_to_spot_queen = format!("{}1q",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_rook = format!("{}1r",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_knight = format!("{}1n",left_spot.chars().nth(0).unwrap());
                                let left_to_spot_bishop = format!("{}1b",left_spot.chars().nth(0).unwrap());
                                unvalidated_moves_vec.push(left_to_spot_queen);
                                unvalidated_moves_vec.push(left_to_spot_rook);
                                unvalidated_moves_vec.push(left_to_spot_knight);
                                unvalidated_moves_vec.push(left_to_spot_bishop);
                            }
                            unvalidated_moves
                            .entry(spot)
                            .or_insert_with(|| Vec::new())
                            .append(&mut unvalidated_moves_vec);
                        }
                    }
                    PieceType::WhiteRook | PieceType::BlackRook => {
                        let mut unvalidated_moves_rook = get_rook_unvalidated_moves(
                            piece,
                            &self.state.st,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_rook.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteKnight | PieceType::BlackKnight => {
                        let mut unvalidated_moves_knight = get_knight_unvalidated_moves(
                            piece,
                            &self.state.st,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_knight.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteBishop | PieceType::BlackBishop => {
                        let mut unvalidated_moves_bishop = get_bishop_unvalidated_moves(
                            piece,
                            &self.state.st,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_bishop.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteQueen | PieceType::BlackQueen => {
                        let mut unvalidated_moves_queen = get_queen_unvalidated_moves(
                            piece,
                            &self.state.st,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_queen.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    PieceType::WhiteKing | PieceType::BlackKing => {
                        let mut unvalidated_moves_king = get_king_unvalidated_moves(
                            piece,
                            &self.state.st,
                            &chess_notation::index_to_spot(index),
                        )?;
                        for (key, mut value) in unvalidated_moves_king.iter_mut() {
                            unvalidated_moves
                                .entry(key.to_owned())
                                .or_insert_with(|| Vec::new())
                                .append(&mut value);
                        }
                    }
                    _ => {}
                }
            }
            }
        }
       
        Ok(unvalidated_moves)
    }

    pub fn is_move_valid(&self, from_spot: &str, to_spot: &str, promotion_opt: Option<&str>)->Result<(MoveType), chess_errors::ChessErrors> {
        // first determine if piece at from is correct player.
        if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
            if let Some(piece) = &self.state.st[index] {
                if piece.get_player() != self.player{
                    let msg = format!("{}",from_spot);
                    return Err(chess_errors::ChessErrors::WrongPlayer(msg));
                }
            } else {
                let msg = format!("{}",from_spot);
                return Err(chess_errors::ChessErrors::NoPiece(msg));
            }
        }
        //if too spot is current player its invalid
        if let Ok(index) = chess_notation::notation_to_index(&to_spot) {
            if let Some(piece) = &self.state.st[index] {
                if piece.get_player() == self.player{
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PlayerPieceAlreadyThere(msg));
                }
            } 
        }
        if  promotion_opt.is_some() {
            //promotions are only valid from 8th rank for pawn
            let from_row = chess_notation::convert_row(from_spot)?;
            let to_row = chess_notation::convert_row(to_spot)?;
            if self.player == Player::White && to_row !=  0 &&  from_row != 1 {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
            if self.player == Player::Black && to_row != 8 && from_row!= 7 {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
        }

        // the x and y deltas will tell what kind of move it is
        
        let (from_point, to_point) = chess_notation::convert_move_notation_to_xy(from_spot,to_spot)?;
        let delta_x: i8 = (from_point.x as i8 - to_point.x as i8) as i8;
        let delta_y: i8 = (from_point.y as i8 - to_point.y as i8) as i8;
        if delta_x == 0  {
            //vertical
            let dir = {
                if delta_y < 0 {
                    Direction::Down
                }else  {
                    Direction::Up
                }
            }; 
            if delta_y.abs() !=1 {
                self.is_path_blocked(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state.st[index] {
                    if let (_, MoveType::Promotion(new_piece)) = piece.move_vertical(to_spot, from_spot, &self.state.st, delta_y, promotion_opt)?{
                        return Ok(MoveType::Promotion(new_piece));
                    }
                }
                else {
                    return Ok(MoveType::Regular);
                }
            }
        } else if delta_y == 0{
            //Horiz
            let dir = {
                if delta_x < 0 {
                    Direction::Right
                }else  {
                    Direction::Left
                }
            }; 
            if delta_x.abs() !=1 {
                self.is_path_blocked(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state.st[index] {
                    if let (_, _move_type) = piece.move_horizontal(to_spot, &self.state.st, delta_x, promotion_opt,self.castling.clone())?{
                        return Ok(_move_type);
                    }
                }
            }
        }else if delta_x.abs() == delta_y.abs(){
            //diagonal
            //determine dir
            let dir = {
                if delta_x > 0 && delta_y > 0  {
                    Direction::UpLeft
                } else if delta_x > 0 && delta_y < 0 {
                    Direction::DownLeft
                } else if delta_x < 0 && delta_y < 0 {
                    Direction::DownRight
                } else {
                    Direction::UpRight
                }
            };
            if delta_x.abs() != 1 {
                //check pieces between because multiple spaces
                self.is_path_blocked(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state.st[index] {
                    if let Some(piece) = &self.state.st[index] {
                        if let (_, MoveType::Promotion(new_piece)) = piece.move_diagonal(to_spot, &self.state.st, delta_y, promotion_opt, self.en_passant_target.clone())?{
                            return Ok(MoveType::Promotion(new_piece));
                        }else if let (_, MoveType::Enpassant(index)) = piece.move_diagonal(to_spot, &self.state.st, delta_y, promotion_opt, self.en_passant_target.clone())?{
                            return Ok(MoveType::Enpassant(index));
                        }
                    }
                    else {
                        return Ok(MoveType::Regular);
                    }
                }
            }
            // if diagonal deltas must be equal, except for Knight

        }else if (delta_x.abs() == 2 && delta_y.abs() ==1) || (delta_x.abs() == 1 && delta_y.abs() ==2){
            if let Ok(index) = chess_notation::notation_to_index(&from_spot) {
                if let Some(piece) = &self.state.st[index] {
                    piece.move_knight(to_spot, &self.state.st, promotion_opt)?;
                }
            }
        }

        //check for current player in check
        Ok(MoveType::Regular)
    }

    pub fn is_path_blocked(&self, from_spot: &str, to_spot: &str, dir: Direction)-> Result<(), chess_errors::ChessErrors>{
        let mut pos:String = to_spot.to_string();
        loop{
            if let Ok(bounds) = chess_notation::get_bounds(&pos){
                let next_pos_opt=  match dir{
                    Direction::Up => {
                            bounds.bottom
                    },
                    Direction::Down => {
                            bounds.top
                    },
                    Direction::Left => {
                            bounds.right
                    },
                    Direction::Right => {
                            bounds.left
                    },
                    Direction::DownLeft => {
                            bounds.top_right_diag
                    },
                    Direction::UpLeft => {
                            bounds.bottom_right_diag
                    },
                    Direction::UpRight => {
                            bounds.bottom_left_diag
                    },
                    Direction::DownRight => {
                            bounds.top_left_diag
                    },
                };
                if let Some(next_pos_array) = next_pos_opt {
                    let x = std::str::from_utf8(&next_pos_array).unwrap();
                    pos= x.to_string();
                    if pos == from_spot {
                        break;
                    }
                    if let Ok(index) = chess_notation::notation_to_index(&pos) {
                        if let Some(piece) = &self.state.st[index] {
                            return Err(chess_errors::ChessErrors::PieceBetween(pos));
                        }
                    }
                }else {
                    return Err(chess_errors::ChessErrors::InvalidNotation(pos.to_string()));
                }
            } else {
                return Err(chess_errors::ChessErrors::InvalidNotation(pos.to_string()));
            }
           
        }
        Ok(())
    }
    
}
