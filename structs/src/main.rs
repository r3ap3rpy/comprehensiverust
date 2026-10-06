struct Move {
    delta: (i32,i32),
    repeat: u32
}
fn main() {
    let m: Move = Move {delta: (10,0),repeat: 5};
    match m {
        Move {delta:(10,0),repeat: 5} => println!("outstanding move!"),
        _ => println!("Maybe next time!"),
    }
}
