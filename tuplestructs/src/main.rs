struct Point(i32,i32);

fn main() {
    let point = Point(10,20);
    println!("({},{})",point.0,point.1);
}
