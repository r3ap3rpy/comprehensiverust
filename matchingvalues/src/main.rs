fn main() {
    let input = 'x';

    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving!"),
        '0' ..='9' => println!("Number input!"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Anything else!"),
    }

    let another_input = 'q';
    match another_input {
        key if key.is_uppercase() => println!("Uppercase!"),
        key => if another_input == 'q' { println!("Quitting") },
        _ => println!("Mastercard!"),
    }
}
