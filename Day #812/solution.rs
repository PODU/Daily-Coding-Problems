// Simplified Elo rating system. Expected score logistic, K=32 update on win/loss.
// record_game adjusts both players. Time O(1) per game, Space O(players).
use std::collections::HashMap;

const K: f64 = 32.0;

struct Elo {
    ratings: HashMap<String, f64>,
}

impl Elo {
    fn new() -> Self {
        Elo { ratings: HashMap::new() }
    }
    fn add(&mut self, name: &str, r: f64) {
        self.ratings.insert(name.to_string(), r);
    }
    fn expected(ra: f64, rb: f64) -> f64 {
        1.0 / (1.0 + 10f64.powf((rb - ra) / 400.0))
    }
    fn record_game(&mut self, winner: &str, loser: &str) {
        let ra = self.ratings[winner];
        let rb = self.ratings[loser];
        let ea = Elo::expected(ra, rb);
        let eb = Elo::expected(rb, ra);
        self.ratings.insert(winner.to_string(), ra + K * (1.0 - ea));
        self.ratings.insert(loser.to_string(), rb + K * (0.0 - eb));
    }
}

fn main() {
    let mut e = Elo::new();
    e.add("A", 1200.0);
    e.add("B", 1200.0);
    println!("Initial: A={:.2} B={:.2}", e.ratings["A"], e.ratings["B"]);
    e.record_game("B", "A");
    println!("After B beats A (equal): A={:.2} B={:.2}", e.ratings["A"], e.ratings["B"]);

    let mut e2 = Elo::new();
    e2.add("C", 1000.0);
    e2.add("D", 1600.0);
    println!("Initial: C={:.2} D={:.2}", e2.ratings["C"], e2.ratings["D"]);
    e2.record_game("C", "D");
    println!("After underdog C beats D: C={:.2} D={:.2}", e2.ratings["C"], e2.ratings["D"]);
    println!("Underdog gained {:.2} vs even-match gain {:.2}",
        e2.ratings["C"] - 1000.0, e.ratings["B"] - 1200.0);
}
