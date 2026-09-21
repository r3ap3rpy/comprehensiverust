fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n-1)+fibonacci(n-2);
    }
}
fn main() {
    // debug will panic at 48, release will wrap meaning incorrect result
    println!("The 10th fibonacci number is {}",fibonacci(48));
}
