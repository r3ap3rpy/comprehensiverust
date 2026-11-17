fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}",s1.len(),s1.capacity());
    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    println!("s2: len = {}, capacity = {}",s2.len(),s2.capacity());
    let s3 = String::from("Switzerland!");
    println!("s3: len = {}, capacity = {}",s3.len(),s3.capacity());
}

