//! Rest Based Interface for a web Gui, that adheres to UCI protocol.
//!
//!
//!
// #[macro_use]
// extern crate serde;

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use oxymcts::chess::chess::{get_legal_moves, game_move_piece, WebGame, FEN_INITIAL_STATE};
use serde::{Deserialize, Serialize};

#[get("/chess")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ValidMovesRequest {
    pub fen_state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidMovesResponse {
    pub moves: HashMap<String,Vec<String>>,
    pub web_game: WebGame,
}

#[post("/valid_moves")]
async fn valid_moves(req: Json<ValidMovesRequest>) -> impl Responder {
    
    let  (moves_map,web_game) = get_legal_moves(&req.fen_state.as_ref().unwrap());
    
    
    let mut resp = ValidMovesResponse {
        moves: moves_map,
        web_game: web_game,
    };
    HttpResponse::Ok().json(resp)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveRequest {
    pub current_fen_state: Option<String>,
    pub chess_move: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveResponse {
    pub resulting_fen: String,
    pub web_game: WebGame,
    moves: HashMap<String,Vec<String>>,
}

#[post("/move_req")]
async fn move_piece(req: Json<MoveRequest>) -> impl Responder {
    
    let  (fen_string, web_game, moves_map) = game_move_piece(&req.current_fen_state.as_ref().unwrap(), &req.chess_move.as_ref().unwrap());
    
    
    let mut resp = MoveResponse {
        resulting_fen: fen_string,
        web_game: web_game,
        moves: moves_map,
    };
    HttpResponse::Ok().json(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive() )
            .service(hello)
            .service(valid_moves)
            .service(move_piece)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await

}
