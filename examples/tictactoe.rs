use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fmt;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

use lib_mcts::{DefaultMcts, GameTrait};

#[derive(Debug, Clone, Default)]
struct TicTacToe {
    /// true cross, false circle
    turn: u8,
    grid: Vec<Vec<u8>>,
    sums_cols: Vec<(usize, usize)>,
    sums_diags: [(usize, usize); 2],
    sums_rows: Vec<(usize, usize)>,
    coordinates_2nd_diag: HashSet<(usize, usize)>,
    n: usize,
}

impl TicTacToe {
    fn new(n: usize) -> Self {
        let mut coordinates_2nd_diag = vec![(n - 1, 0)];
        for i in 0..=n - 2 {
            let (x, y) = coordinates_2nd_diag[i];
            coordinates_2nd_diag.push((x - 1, y + 1))
        }
        let coordinates_2nd_diag = coordinates_2nd_diag.into_iter().collect();
        TicTacToe {
            turn: 1,
            grid: vec![vec![0; n]; n],
            sums_cols: vec![(0, 0); n],
            sums_diags: [(0, 0); 2],
            sums_rows: vec![(0, 0); n],
            coordinates_2nd_diag,
            n,
        }
    }

    pub fn get_turn(&self) -> u8 {
        self.turn
    }

    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let l = self.grid.len();
        for i in 0..l {
            for j in 0..l {
                if self.grid[i][j] == 0 {
                    res.push((i, j))
                }
            }
        }
        res
    }

    pub fn play(&mut self, p: (usize, usize)) {
        self.grid[p.0][p.1] = if self.turn == 1 {
            self.sums_rows[p.0].0 += 1;
            self.sums_cols[p.1].0 += 1;
            if p.0 == p.1 {
                self.sums_diags[0].0 += 1;
            }
            if self.coordinates_2nd_diag.contains(&p) {
                self.sums_diags[1].0 += 1;
            }
            self.turn = 2;
            1
        } else {
            self.sums_rows[p.0].1 += 1;
            self.sums_cols[p.1].1 += 1;
            if p.0 == p.1 {
                self.sums_diags[0].1 += 1;
            }
            if self.coordinates_2nd_diag.contains(&p) {
                self.sums_diags[1].1 += 1;
            }
            self.turn = 1;
            2
        };
    }

    pub fn is_final(&self) -> bool {
        self.get_winner() != 0 || self.legal_moves().is_empty()
    }

    /// Return non if the game nobody won
    pub fn get_winner(&self) -> u8 {
        self.sums_cols
            .iter()
            .chain(self.sums_diags.iter())
            .chain(self.sums_rows.iter())
            .find(move |x| x.0 == self.n || x.1 == self.n)
            .map(move |x| match x {
                (y, _) if self.n == *y => 1,
                (_, y) if self.n == *y => 2,
                _ => unreachable!(),
            }).unwrap_or(0)
    }
}

impl GameTrait for TicTacToe {
    type Player = u8;
    type Move = (usize, usize);

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
        TicTacToe::is_final(self)
    }

    fn do_move(&mut self, m: &Self::Move) {
        self.play(*m);
    }

    fn get_winner(&self) -> Self::Player {
        TicTacToe::get_winner(self)
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str_final = String::new();
        for i in 0..self.n {
            str_final.push_str("|");
            for j in 0..self.n {
                let s = match self.grid[i][j] {
                    0 => " ",
                    1 => "X",
                    2 => "0",
                    _ => unreachable!()
                };
                str_final.push_str(s);
                str_final.push_str("|");
            }
            str_final.push_str("\n");
        }
        write!(f, "{}", str_final)
    }
}

fn mcts_agent<Game: GameTrait>(state: &Game, playouts: usize) -> Game::Move {
    let mut mcts = DefaultMcts::new(state);
    for _ in 0..playouts {
        mcts.execute(());
    }
    mcts.best_move()
}

fn random_agent<Game: GameTrait>(state: &Game) -> Game::Move {
    state.legals_moves().choose(&mut thread_rng()).unwrap().clone()
}

fn run_a_game(n: usize) -> u8 {
    let mut tictactoe = TicTacToe::new(n);
    while !tictactoe.is_final() {
        let move_random = random_agent(&tictactoe);
        tictactoe.play(move_random);
        if !tictactoe.is_final() {
            let move_mcts = mcts_agent(&tictactoe, 1000);
            tictactoe.play(move_mcts);
        }
    }
    return tictactoe.get_winner();
}

fn main() {
    println!("Player 1: Cross");
    println!("Player 2: Circle");
    let mut tictactoe = TicTacToe::new(6);
    while !tictactoe.is_final() {
        println!("Random turn: ");
        let move_random = dbg!(random_agent(&tictactoe));
        tictactoe.play(move_random);
        println!("{}", tictactoe);
        if !tictactoe.is_final() {
            println!("Mcts turn: ");
            let move_mcts = dbg!(mcts_agent(&tictactoe, 10000));
            tictactoe.play(move_mcts);
            println!("{}", tictactoe);
        }
    }
    println!("Winner: {}", tictactoe.get_winner());
    //println!("{}", mcts.write_tree());
    let number_of_games = 1000;
    let stats = (0..number_of_games)
        .into_par_iter()
        .map(|_| {
            let idx = run_a_game(5) as usize;
            let mut arr = [0, 0, 0];
            arr[idx] = 1;
            arr
        })
        .reduce(|| [0, 0, 0], |acc, x| {
            [acc[0] + x[0], acc[1] + x[1], acc[2] + x[2]]
        });

    println!("With C = sqrt(2), 10000 rollouts, in a tictactoe of dim 5, and versus a random bot \
    the mcts wins {}% of time and there is {}% nulls", (stats[2] as f64 / number_of_games as f64)
        * 100., (stats[0] as f64 / number_of_games as f64) * 100.)
}



