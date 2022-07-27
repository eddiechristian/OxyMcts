use std::{collections::HashMap, thread};

//use oxymcts::chess::chess::get_game_result;

const NTHREADS: u32 = 2;

#[derive(Debug, PartialEq, Clone)]
pub enum PieceType {
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
pub struct Piece {
    pub piece_type: PieceType,
    moves: HashMap::<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ChessState {
    pub st: [Option<Piece>; 64],
}
pub const INIT: Option<Piece> = None;

use rayon::prelude::*;
fn main() {
    let mut state: [Option<Piece>; 64] = [INIT; 64];
    let mut index =0;
    state[index] = Some(Piece {
        piece_type: PieceType::BlackPawn,
        moves: HashMap::<String, Vec<String>>::new(),
    });
    index += 1;
    state[index] = Some(Piece {
        piece_type: PieceType::WhitePawn,
        moves: HashMap::<String, Vec<String>>::new(),
    });
    index += 1;
    state[index] = Some(Piece {
        piece_type: PieceType::WhiteBishop,
        moves: HashMap::<String, Vec<String>>::new(),
    });

    index += 1;
    state[index] = Some(Piece {
        piece_type: PieceType::WhiteRook,
        moves: HashMap::<String, Vec<String>>::new(),
    });
    index += 1;

    let mut unvalidated_moves = HashMap::<String, Vec<String>>::new();
    state.par_iter_mut() 
        .for_each(|piece_opt:&mut Option<Piece>|  {
            match piece_opt {
                Some(piece) => {
                    match piece.piece_type {
                        PieceType::BlackPawn => {
                            
                            piece.moves.insert("a2".to_owned(), vec!["a3".to_owned(), "a4".to_owned()]);
                        },
                        PieceType::WhiteRook => {
                            piece.moves.insert("b2".to_owned(), vec!["b3".to_owned(), "b4".to_owned()]);
                        }
                        _ => {
                        }
                    }
                }, 
                None => {
                }
            }
         });
     state.par_iter() 
         .for_each(|piece_opt: &Option<Piece>|
            {
                if let Some(piece) = piece_opt {
                    println!("moves {:?}", piece.moves);
                }
            });
    
            state.par_iter_mut() 
        .for_each(|piece_opt:&mut Option<Piece>|  {
            match piece_opt {
                Some(piece) => {
                    match piece.piece_type {
                        PieceType::BlackPawn => {
                            piece.moves.clear();
                            piece.moves.insert("j2".to_owned(), vec!["k3".to_owned(), "a4".to_owned()]);
                        },
                        PieceType::WhiteRook => {
                            piece.moves.clear();
                            piece.moves.insert("j2".to_owned(), vec!["x3".to_owned(), "q4".to_owned()]);
                        }
                        _ => {
                        }
                    }
                }, 
                None => {
                }
            }
         });

         state.par_iter() 
         .for_each(|piece_opt: &Option<Piece>|
            {
                if let Some(piece) = piece_opt {
                    println!("moves {:?}", piece.moves);
                }
            });
}


// This is the `main` thread
// fn main() {
//     // Make a vector to hold the children which are spawned.
//     let mut children = vec![];
//     let mut unvalidated_moves = HashMap::<String, Vec<String>>::new();
//     for i in 0..NTHREADS {
//         // Spin up another thread
//         children.push(thread::spawn(move || {
//             if i ==0 {
//                 let mut unvalidated_moves = HashMap::<String,Vec<String>>::new();
    
//                 unvalidated_moves.insert("a2".to_owned(), vec!["a3".to_owned(), "a4".to_owned()]);
//                 return unvalidated_moves;
//             } else if i ==1 {
//                 let mut unvalidated_moves = HashMap::<String,Vec<String>>::new();
//                 unvalidated_moves.insert("b2".to_owned(), vec!["b3".to_owned(), "b4".to_owned()]);
//                 return unvalidated_moves;
//             } else {
//                 let mut unvalidated_moves = HashMap::<String,Vec<String>>::new();
//                 return unvalidated_moves;
//             }
//         }));
//     }

//     for child in children {
//         // Wait for the thread to finish. Returns a result.
//         let mut map = child.join().unwrap();
//         for (key, mut value) in map.iter_mut() {
//             unvalidated_moves
//                 .entry(key.to_owned())
//                 .or_insert_with(|| Vec::new())
//                 .append(&mut value);
//         }
//     }
//     println!("unvalidated_moves {:?}",unvalidated_moves);
// }

