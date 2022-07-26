use std::collections::HashMap;

use super::{
    chess::Piece,
    chess_errors,
    chess_notation::{self},
};

pub(crate) fn get_knight_unvalidated_moves(
    piece: &Piece,
    state: &[Option<Piece>; 64],
    spot: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let mut unvalidated_moves = HashMap::new();

    let col_spot: u8 = (chess_notation::convert_col(spot)?) as u8;
    let row_spot: u8 = (chess_notation::convert_row(spot)?) as u8;
    let row_minus2 = match row_spot {
        2..=7 => Some(row_spot - 2),
        _ => None,
    };
    let row_minus1 = match row_spot {
        1..=7 => Some(row_spot - 1),
        _ => None,
    };
    let col_minus2 = match col_spot {
        2..=7 => Some(col_spot - 2),
        _ => None,
    };
    let col_minus1 = match col_spot {
        1..=7 => Some(col_spot - 1),
        _ => None,
    };
    let row_plus2 = match row_spot {
        0..=5 => Some(row_spot + 2),
        _ => None,
    };
    let row_plus1 = match row_spot {
        0..=6 => Some(row_spot + 1),
        _ => None,
    };
    let col_plus2 = match col_spot {
        0..=5 => Some(col_spot + 2),
        _ => None,
    };
    let col_plus1 = match col_spot {
        0..=6 => Some(col_spot + 1),
        _ => None,
    };
    //up 2 right 1
    if row_minus2.is_some() && col_plus1.is_some() {
        let up2rt1_index = row_minus2.unwrap() * 8 + col_plus1.unwrap();
        let mut to_spot = chess_notation::index_to_spot(up2rt1_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //up 1 right 2
    if row_minus1.is_some() && col_plus2.is_some() {
        let up1rt2_index = row_minus1.unwrap() * 8 + col_plus2.unwrap();
        let mut to_spot = chess_notation::index_to_spot(up1rt2_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //up 2 left 1
    if row_minus2.is_some() && col_minus1.is_some() {
        let up2lf1_index = row_minus2.unwrap() * 8 + col_minus1.unwrap();
        let mut to_spot = chess_notation::index_to_spot(up2lf1_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //up 1 left 2
    if row_minus1.is_some() && col_minus2.is_some() {
        let up1lf2_index = row_minus1.unwrap() * 8 + col_minus2.unwrap();
        let mut to_spot = chess_notation::index_to_spot(up1lf2_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //down 2 right 1
    if row_plus2.is_some() && col_plus1.is_some() {
        let dn2rt1_index = row_plus2.unwrap() * 8 + col_plus1.unwrap();
        let mut to_spot = chess_notation::index_to_spot(dn2rt1_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //down 1 right 2
    if row_plus1.is_some() && col_plus2.is_some() {
        let dn1rt2_index = row_plus1.unwrap() * 8 + col_plus2.unwrap();
        let mut to_spot = chess_notation::index_to_spot(dn1rt2_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //down 2 left 1
    if row_plus2.is_some() && col_minus1.is_some() {
        let dn2lf1_index = row_plus2.unwrap() * 8 + col_minus1.unwrap();
        let mut to_spot = chess_notation::index_to_spot(dn2lf1_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    //down 1 left 2
    if row_plus1.is_some() && col_minus2.is_some() {
        let dn1lf2_index = row_plus1.unwrap() * 8 + col_minus2.unwrap();
        let mut to_spot = chess_notation::index_to_spot(dn1lf2_index as usize);
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(to_spot.to_string());
    }
    Ok(unvalidated_moves)
}
