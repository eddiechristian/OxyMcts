use oxymcts::chess::chess::get_game_result;

pub fn main() {
    let x = get_game_result("8/1b3k2/2p1QPpB/1pP1K3/1nP5/p7/4Np2/3RR3 b k - 1 123");
    println!("{:?}", x); 
}