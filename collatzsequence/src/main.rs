fn collatz(mut n: i32) -> i32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 {n / 2} else {3 * n + 1};
        len += 1;
    }
    len
}
fn main() {
    println!("Collatz(10) = {}",collatz(10));
}
