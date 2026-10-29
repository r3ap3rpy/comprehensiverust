struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}
trait Pet {
    fn talk(&self) -> String;
}
impl Pet for Dog {
    fn talk(&self) -> String {
        format!("{} says woof woof!",self.name)
    }
}
impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Meooow!")
    }
}
fn generic_pet(pet: &impl Pet) {
    println!("{}",pet.talk());
}
fn dynamic_pet(pet: &dyn Pet) {
    println!("{}",pet.talk());
}
fn main() {
    let cat = Cat {lives: 9};
    let dog = Dog {name: String::from("Döme"),age: 15};
    generic_pet(&cat);
    generic_pet(&dog);
    dynamic_pet(&cat);
    dynamic_pet(&dog);
}
