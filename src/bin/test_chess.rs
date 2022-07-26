use oxymcts::chess::chess::get_game_result;

pub fn main() {
    let x = get_game_result("1nb2bnr/4pk2/7p/p2p1ppP/p2P1P1R/2q1PN2/1PP3P1/RN2KB2 w Qk - 1 34");
    println!("{:?}", x); 
}