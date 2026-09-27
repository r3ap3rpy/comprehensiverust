fn main() {
    let s1: &str = "Hello World!";

    let mut s2: String = String::from("Hello");
    s2.push_str(" ");
    s2.push_str("World!");
    println!("s1: {s1:?}");
    println!("s2: {s2:?}");
}
