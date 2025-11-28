// Day 667: Simplified Elo rating. Expected score E = 1/(1+10^((Rb-Ra)/400)),
// update R += K*(actual - expected). Underdog gains more. Each update O(1).
use std::collections::HashMap;

struct Elo {
    k: f64,
    start: f64,
    r: HashMap<String, f64>,
}

impl Elo {
    fn new() -> Self { Elo { k: 32.0, start: 1200.0, r: HashMap::new() } }
    fn rating(&mut self, p: &str) -> f64 {
        *self.r.entry(p.to_string()).or_insert(self.start)
    }
    fn game(&mut self, winner: &str, loser: &str) {
        let ra = self.rating(winner);
        let rb = self.rating(loser);
        let ea = 1.0 / (1.0 + 10f64.powf((rb - ra) / 400.0));
        let eb = 1.0 - ea;
        self.r.insert(winner.to_string(), ra + self.k * (1.0 - ea));
        self.r.insert(loser.to_string(), rb + self.k * (0.0 - eb));
    }
}

fn main() {
    let mut e = Elo::new();
    e.r.insert("A".into(), 1200.0);
    e.r.insert("B".into(), 2000.0);
    e.game("A", "B");
    println!("A: {:.1}", e.rating("A")); // ~1231.5
    println!("B: {:.1}", e.rating("B")); // ~1968.5
}
