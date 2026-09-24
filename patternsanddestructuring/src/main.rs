fn check_tuple_order(tuple: (i32,i32,i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && right > middle
}
fn main() {
    let tuple = (1,5,3);
    println!("{:?} : {:?}",tuple, check_tuple_order(tuple));
}
