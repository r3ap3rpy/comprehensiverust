enum Result {
    Ok(i32),
    Err(String)
}
fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("Cannot divide {n} into two equal parts!"))
    }
}
fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in half is {half}"),
        Result::Err(error) => eprintln!("The error was: {error}"),
    }
}
