fn get_index() -> usize {
    6
}
fn main() {
    let mut a: [i8;5] = [5,4,3,2,1];
    a[2] = 0;
    println!("a: {a:?}");
    //a[get_index()] = 10;
    //println!("a: {a:?}");
}
