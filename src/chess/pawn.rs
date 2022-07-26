use std::collections::HashMap;

use super::{
    chess::{Piece, MoveType, PieceType, Player},
    chess_errors,
    chess_notation::{self},
};

pub(crate) fn get_pawn_unvalidated_moves(
    piece: &Piece,
    state: &[Option<Piece>; 64],
    spot: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let mut unvalidated_moves = HashMap::new();

    let bounds = chess_notation::get_bounds(spot)?;
    if let Ok(index) = chess_notation::notation_to_index(&spot) {
        if let Some(piece) = &state[index] {
            if piece.get_player() == Player::White {
                let (top_opt, top_right_opt, top_left_opt) =
                    (bounds.top, bounds.top_right_diag, bounds.top_left_diag);
                if let Some(top_array) = top_opt {
                    let top = std::str::from_utf8(&top_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(top.to_string());
                    let next_bounds = chess_notation::get_bounds(top)?;
                    if let Some(next_top_array) = next_bounds.top {
                        let next_top = std::str::from_utf8(&next_top_array).unwrap();
                        unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(next_top.to_string());
                    }
                }
                if let Some(top_right_array) = top_right_opt {
                    let top_right = std::str::from_utf8(&top_right_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(top_right.to_string());
                }
                if let Some(top_left_array) = top_left_opt {
                    let top_left = std::str::from_utf8(&top_left_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(top_left.to_string());
                }
            } else if piece.get_player() == Player::Black {
                let (bottom_opt, bottom_right_opt, bottom_left_opt) = (
                    bounds.bottom,
                    bounds.bottom_right_diag,
                    bounds.bottom_left_diag,
                );
                if let Some(bottom_array) = bottom_opt {
                    let bottom = std::str::from_utf8(&bottom_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(bottom.to_string());
                    let next_bounds = chess_notation::get_bounds(bottom)?;
                    if let Some(next_bottom_array) = next_bounds.bottom {
                        let next_bottom = std::str::from_utf8(&next_bottom_array).unwrap();
                        unvalidated_moves
                            .entry(spot.to_owned())
                            .or_insert_with(|| Vec::new())
                            .push(next_bottom.to_string());
                    }
                }
                if let Some(bottom_right_array) = bottom_right_opt {
                    let bottom_right = std::str::from_utf8(&bottom_right_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(bottom_right.to_string());
                }
                if let Some(bottom_left_array) = bottom_left_opt {
                    let bottom_left = std::str::from_utf8(&bottom_left_array).unwrap();
                    unvalidated_moves
                        .entry(spot.to_owned())
                        .or_insert_with(|| Vec::new())
                        .push(bottom_left.to_string());
                }
            }
        }
    }

    Ok(unvalidated_moves)
}

pub(crate) fn  move_pawn_vertical(piece: &Piece, to_spot: &str, from_spot: &str, state: &[Option<Piece>; 64], delta_y: i8, promotion_opt: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
    if let Ok(index) = chess_notation::notation_to_index(&to_spot) {
        if  let Some(forward_piece) = &state[index]{
            if forward_piece.get_player() != piece.get_player(){
                 //pawns cannot attack forward
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::PawnCantAttackForward(msg));
            }
        }
    }
    if delta_y.abs() > 2 {
        //pawns cannot move vert more than 2
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    if delta_y.abs() == 2 {
         //pawns cannot move vert more than 1, if they moved before
         let row = chess_notation::convert_row(from_spot)?;
         if piece.get_player() == Player::White && row != 6 {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
         }else if piece.get_player() == Player::Black && row != 1 {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
         }
        
    }
    if piece.get_player() == Player::Black && delta_y > 0 {
        //black pawn cannot move up
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    } else if piece.get_player() == Player::White && delta_y < 0 {
        //white pawn cannot move down
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    //check for promotion
    if let Ok(row) =chess_notation::convert_row(to_spot){
        if piece.get_player() == Player::Black {
            if row == 7 {
                match promotion_opt {
                    None => {
                        return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackQueen)))
                    },
                    Some(promotion) => {
                        match promotion {
                            "r" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackRook)));
                            },
                            "b" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackBishop)));
                            },
                            "k" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackKnight)));
                            },
                            _ => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackQueen)));
                            },
                        }
                    }
                }
            }
        } else {
            if row == 0 {
                match promotion_opt {
                    None => {
                        return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteQueen)))
                    },
                    Some(promotion) => {
                        match promotion {
                            "r" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteRook)));
                            },
                            "b" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteBishop)));
                            },
                            "k" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteKnight)));
                            },
                            _ => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteQueen)));
                            },
                        }
                        
                    }
                }
            }
        }   
    }
    Ok((to_spot.to_string(),MoveType::Regular))
}
pub(crate) fn  move_pawn_diagonal( piece: &Piece, to_spot: &str, state: &[Option<Piece>; 64], delta_y: i8, promotion_opt: Option<&str>, enpassant_target: Option<String>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
    if piece.get_player() == Player::Black && delta_y > 0 {
        //black pawn annot move up
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    } else if piece.get_player() == Player::White && delta_y < 0 {
        //white pawn cannot move down
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    if let Ok(index) = chess_notation::notation_to_index(&to_spot) {
        if  state[index].is_none() {
            //check for enpassant
            if let Some( target) = enpassant_target {
                if to_spot == target {
                    if  piece.get_player() == Player::White {
                        let mut attack_index = chess_notation::notation_to_index(to_spot)?;
                        attack_index +=8;
                        return Ok((to_spot.to_string(),MoveType::Enpassant(attack_index)));
                    }else {
                        let mut attack_index = chess_notation::notation_to_index(to_spot)?;
                        attack_index -=8;
                        return Ok((to_spot.to_string(),MoveType::Enpassant(attack_index)));
                    }
                    
                }else {
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PawnCanOnlyAttackDiagonal(msg));
                }
            }
            else {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::PawnCanOnlyAttackDiagonal(msg));
            }
        }else if state[index].as_ref().unwrap().get_player() != piece.get_player(){
            return Ok((to_spot.to_string(),MoveType::Regular));
        }else {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::PawnCanOnlyAttackDiagonal(msg));
        }
    }
    if let Ok(row) =chess_notation::convert_row(to_spot){
        if piece.get_player() == Player::Black {
            if row == 7 {
                match promotion_opt {
                    None => {
                        return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackQueen)))
                    },
                    Some(promotion) => {
                        match promotion {
                            "r" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackRook)));
                            },
                            "b" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackBishop)));
                            },
                            "k" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackKnight)));
                            },
                            _ => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::BlackQueen)));
                            },
                        }
                    }
                }
            }
        } else {
            if row == 0 {
                match promotion_opt {
                    None => {
                        return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteQueen)))
                    },
                    Some(promotion) => {
                        match promotion {
                            "r" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteRook)));
                            },
                            "b" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteBishop)));
                            },
                            "k" => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteKnight)));
                            },
                            _ => {
                                return Ok((to_spot.to_string(),MoveType::Promotion(PieceType::WhiteQueen)));
                            },
                        }
                        
                    }
                }
            }
        }   
    }
    Ok((to_spot.to_string(),MoveType::Regular))
}