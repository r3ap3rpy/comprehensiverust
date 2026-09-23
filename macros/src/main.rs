fn factorial(n: i32) -> i32 {
    let mut product = 1;
    for i in 1..n {
        product *= dbg!(i);
    }
    product
}
fn fizzbuzz(n: u32) -> u32 {
    todo!();
}
fn main() {
    println!("10! = {}",factorial(10));
    fizzbuzz(10);
}
