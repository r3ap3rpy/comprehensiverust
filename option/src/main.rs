fn main() {
    let name = "Szabó Dániel Ernő";
    let mut position: Option<usize> = name.find("ó");
    dbg!(position);
    assert_ne!(position.unwrap(),14);
    position = name.find("á");
    dbg!(position);
 
}
