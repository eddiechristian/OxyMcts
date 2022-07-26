use std::collections::HashMap;

use super::{chess::Piece, chess_errors, chess_notation};

pub(crate) fn get_queen_unvalidated_moves(
    piece: &Piece,
    state: &[Option<Piece>; 64],
    spot: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    let mut unvalidated_moves = chess_notation::get_unvalidated_diag_moves(spot)?;
    let mut horiz_vert_map = chess_notation::get_unvalidated_horiz_vert_moves(spot)?;

    for (key, mut value) in horiz_vert_map.iter_mut() {
        unvalidated_moves
            .entry(key.to_owned())
            .or_insert_with(|| Vec::new())
            .append(&mut value);
    }

    Ok(unvalidated_moves)
}
