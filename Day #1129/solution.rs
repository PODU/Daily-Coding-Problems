// Simplified Elo rating system. Expected score Ea=1/(1+10^((Rb-Ra)/400)),
// update Ra'=Ra+K*(Sa-Ea), K=32. recordGame transfers points winner<-loser. O(1) per game.
use std::collections::HashMap;

const K: f64 = 32.0;

struct EloSystem {
    rating: HashMap<String, f64>,
}

impl EloSystem {
    fn new() -> Self { EloSystem { rating: HashMap::new() } }
    fn add_player(&mut self, name: &str) {
        self.rating.entry(name.to_string()).or_insert(1200.0);
    }
    fn expected(ra: f64, rb: f64) -> f64 { 1.0 / (1.0 + 10f64.powf((rb - ra) / 400.0)) }
    fn record_game(&mut self, winner: &str, loser: &str) {
        self.add_player(winner);
        self.add_player(loser);
        let ra = self.rating[winner];
        let rb = self.rating[loser];
        let ea = Self::expected(ra, rb);
        let eb = Self::expected(rb, ra);
        self.rating.insert(winner.to_string(), ra + K * (1.0 - ea));
        self.rating.insert(loser.to_string(), rb + K * (0.0 - eb));
    }
    fn get(&self, name: &str) -> i64 { self.rating[name].round() as i64 }
}

fn main() {
    let mut elo = EloSystem::new();
    elo.add_player("A");
    elo.rating.insert("A".to_string(), 1200.0);
    elo.add_player("B");
    elo.rating.insert("B".to_string(), 2000.0);
    println!("Before: A={}, B={}", elo.get("A"), elo.get("B"));
    elo.record_game("A", "B"); // lower-rated A beats higher-rated B
    println!("After A(1200) beats B(2000): A={}, B={}", elo.get("A"), elo.get("B"));
}
