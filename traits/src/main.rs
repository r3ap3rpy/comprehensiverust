trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("Oh you are cute, what is your name: {}",self.talk());
    }
}
struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}, I am {} years old!",self.name, self.age)
    }
}

fn main() {
    let dog = Dog { name: String::from("Döme"), age: 15};
    dog.greet();
}
