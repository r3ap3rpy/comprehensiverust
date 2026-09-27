fn main() {
    let a: [i32;6] = [1,2,3,4,5,6];
    println!("a: {a:?}");
    let b: &[i32] = &a[2..4];
    println!("b: {b:?}");
}
