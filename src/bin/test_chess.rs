use oxymcts::chess::chess::get_game_result;

pub fn main() {
    let x = get_game_result("rn1qk1nr/pb1p4/Bpp3p1/4Pp1Q/3bPB1P/N6R/PPP2PP1/3RK1N1 b kq - 5 25");
    println!("{:?}", x); 
}