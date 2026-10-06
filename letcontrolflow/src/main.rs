use std::time::Duration;

fn sleep_for(secs: f32) {
    let result = Duration::try_from_secs_f32(secs);
    if let Ok(duration) = result {
        std::thread::sleep(duration);
        println!("Slept for {duration:?} seconds!");
    }
}
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got none"))
    };
    let first_byte_char = if let Some(first) = s.chars().next() {
        first
    } else {
        return Err(String::from("got empty string"));
    };
    let digit = if let Some(digit) = first_byte_char.to_digit(16) {
        digit
    } else {
        return Err(String::from("not a hex digit"));
    };
    Ok(digit)
}
fn main() {
    sleep_for(-10.0);
    sleep_for(2.0);

    let mut name = String::from("Daniel");
    while let Some(c) = name.pop() {
        println!("Popping some: {c:?}");
    }
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
