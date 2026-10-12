trait Animal {
    fn leg_count(&self) -> u8;
}

trait Pet: Animal {
    fn name(&self) -> String;
}
struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u8 {
        4
    }
}
impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}
fn main() {
    let puppy = Dog(String::from("Bab"));
    println!("{} has {} legs",puppy.name(),puppy.leg_count());
}
