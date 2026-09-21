fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a*b + b*c + c*a;
}
fn main() {
    // i32 -> i16 panics in a debug build and wraps in a release build
    println!("The interproduct result: {}",interproduct(120, 100, 240));
}
