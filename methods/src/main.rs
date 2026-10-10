#[derive(Debug)]
struct RaceCar {
    name: String,
    laps: Vec<i32>,
}

impl RaceCar {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name), laps: Vec::new()
        }
    }
    fn add_laps(&mut self, lap: i32) {
        self.laps.push(lap);
    }
    fn print_laps(&self) {
        println!("Record {} laps for {}", self.laps.len(),self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap: {idx}, lap: {lap} sec");
        }
    }
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} finished with total {} lap time!", self.name, total);
    }
}

fn main() {
    let mut mustang = RaceCar::new("Ford Mustang");
    mustang.add_laps(10);
    mustang.add_laps(20);
    mustang.add_laps(99);
    mustang.print_laps();
    mustang.finish();
}
