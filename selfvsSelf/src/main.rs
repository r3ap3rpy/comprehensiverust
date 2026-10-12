
struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let daniel = User::new(String::from("Daniel"));
    daniel.greet();
}
