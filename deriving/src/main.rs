#[derive(Debug, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            name: self.name.clone(),
            strength: self.strength.clone(),
            hit_points: self.hit_points.clone(),
        }
    }
}

fn main() {
    let p1 = Player::default(); // Default trait
    let mut p2 = p1.clone(); // Clone
    p2.name = String::from("r3ap3rpy");
    println!("{p1:?} vs {p2:?}"); // Debug
}
