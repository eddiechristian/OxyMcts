use std::collections::HashMap;

use super::{
    chess::Piece,
    chess_errors,
    chess_notation::{self},
};

pub(crate) fn get_king_unvalidated_moves(
    piece: &Piece,
    state: &[Option<Piece>; 64],
    spot: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let mut unvalidated_moves = HashMap::new();

    let bounds = chess_notation::get_bounds(spot)?;

    if let Some(top_right_array) = bounds.top_right_diag {
        let top_right = std::str::from_utf8(&top_right_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(top_right.to_string());
    }
    if let Some(top_left_array) = bounds.top_left_diag {
        let top_left = std::str::from_utf8(&top_left_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(top_left.to_string());
    }
    if let Some(bottom_left_array) = bounds.bottom_left_diag {
        let bottom_left = std::str::from_utf8(&bottom_left_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(bottom_left.to_string());
    }
    if let Some(bottom_right_array) = bounds.bottom_right_diag {
        let bottom_right = std::str::from_utf8(&bottom_right_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(bottom_right.to_string());
    }
    if let Some(bottom_array) = bounds.bottom {
        let bottom = std::str::from_utf8(&bottom_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(bottom.to_string());
    }
    if let Some(top_array) = bounds.top {
        let top = std::str::from_utf8(&top_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(top.to_string());
    }
    if let Some(left_array) = bounds.left {
        let left = std::str::from_utf8(&left_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(left.to_string());
    }
    if let Some(right_array) = bounds.right {
        let right = std::str::from_utf8(&right_array).unwrap();
        unvalidated_moves
            .entry(spot.to_owned())
            .or_insert_with(|| Vec::new())
            .push(right.to_string());
    }
    //add castling moves
    let col = chess_notation::convert_col(spot).unwrap();
    let row = chess_notation::convert_row(spot).unwrap();
    if col == 4 && row == 7 {
        //white
        unvalidated_moves
        .entry(spot.to_owned())
        .or_insert_with(|| Vec::new())
        .push("g1".to_string());

        unvalidated_moves
        .entry(spot.to_owned())
        .or_insert_with(|| Vec::new())
        .push("c1".to_string());
    } else if  col == 4 && row == 0  {
        //black
        unvalidated_moves
        .entry(spot.to_owned())
        .or_insert_with(|| Vec::new())
        .push("g8".to_string());

        unvalidated_moves
        .entry(spot.to_owned())
        .or_insert_with(|| Vec::new())
        .push("c8".to_string());

    }
    Ok(unvalidated_moves)
}
