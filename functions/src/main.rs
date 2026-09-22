fn gcd(a: i32, b: i32) -> i32 {
    if b > 0 { gcd(b, a % b) } else { a }
}
fn main() {
    dbg!(gcd(143,52));
}
