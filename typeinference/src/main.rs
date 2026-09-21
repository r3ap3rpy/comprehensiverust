fn takes_u32(x: u32) {
    println!("u32: {}",x);
}
fn takes_i8(x: i8){
    println!("i8: {}",x);
}
fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
}
