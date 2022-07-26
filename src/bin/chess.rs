use oxymcts::GameTrait;

use oxymcts::chess::chess::Player;

#[derive(Debug, Clone, Default)]
struct ChessMCTS {
    turn: Player,
    fen_string: String,
}

impl ChessMCTS {
    fn new(n: usize) -> Self {
        
    }

    pub fn get_turn(&self) -> Player {
        self.turn
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
        todo!()
    }

    fn do_move(&mut self, m: &Self::Move) {
        todo!()
    }

    fn get_winner(&self) -> Self::Player {
        todo!()
    }
}


fn main() {

}