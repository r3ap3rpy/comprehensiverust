fn main() {
    let double_it = |n| n * 2;
    println!("{}",double_it(10));
    let add_1f32 = |n: f32| -> f32 { n + 1.0};
    println!("{}",add_1f32(10.));
}
