
fn main() {
    let s = String::new();
    let addr = std::net::Ipv4Addr::from([127,0,0,1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s} {addr} {one} {bigger}");
    let ss: String = "hello".into();
    let aaddr: std::net::Ipv4Addr = [127,0,0,1].into();
    let oone: i16 = true.into();
    let bbigger: i32 = 123_i16.into();
    println!("{ss} {aaddr} {oone} {bbigger}");
}
