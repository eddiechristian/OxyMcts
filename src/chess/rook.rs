use std::collections::HashMap;

use super::{chess::Piece, chess_errors, chess_notation};

pub(crate) fn get_rook_unvalidated_moves(
    piece: &Piece,
    state: &[Option<Piece>; 64],
    spot: &str,
) -> Result<HashMap<String, Vec<String>>, chess_errors::ChessErrors> {
    return chess_notation::get_unvalidated_horiz_vert_moves(spot);
}
