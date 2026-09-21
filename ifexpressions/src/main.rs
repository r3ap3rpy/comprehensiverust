fn main() {
    let x = 10;
    if x == 0 {
        println!("X is zero!");
    } else if x < 100 {
        println!("X is biggish!");
    } else {
        println!("X is monstrous!");
    }

    let y = 20;
    let size = if y == 20 { "small" } else {"big" };
    println!("Size is {:?}",size);
}
