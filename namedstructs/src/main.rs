struct Person {
    name: String,
    age: u8,
}
fn describe(person: &Person) {
    println!("Person name: {}, age: {}",person.name, person.age);
}

fn main() {
    let mut person = Person {
        name: String::from("Daniel"),
        age: 36,
    };
    describe(&person);
    person.age = 99;
    describe(&person);
    let name = String::from("Lenke");
    let age = 2;
    let lenke = Person {name, age};
    describe(&lenke);
}
