fn main() {
    let val = 1;
    match val {
        1 => println!("One"),
        10 => println!("Ten"),
        100 => println!("Onehundred!"),
        _ => println!("Something else!"),
    }

    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };
    println!("val is {}",val);
}
