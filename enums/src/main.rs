#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: i32, y: i32},
}
#[repr(u32)]
enum Bar{
    A,
    B = 1000,
    C,
}
fn main() {
    let player_move: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("move: {player_move:?}");
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}
