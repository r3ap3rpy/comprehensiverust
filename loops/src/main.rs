fn main() {
    let mut x = 200;
    while x >= 0 {
        x -= 10;
        dbg!(x);
    }
    println!("x = {}",x);

    for i in 1..5 {
        dbg!(i);
    }

    for item in [1,2,3,4,5] {
        dbg!(item);
    }

    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 10 {
            break;
        }
    }
}
