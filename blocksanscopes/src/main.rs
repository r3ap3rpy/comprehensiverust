fn main() {
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y 
    };
    println!("{}",x);
    dbg!(x);
}
