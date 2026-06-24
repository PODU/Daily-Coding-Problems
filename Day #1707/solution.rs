// Elo rating: expected = 1/(1+10^((Rb-Ra)/400)); delta = K*(score-expected), zero-sum. O(1) per game.
use std::collections::BTreeMap;

struct EloSystem {
    ratings: BTreeMap<String, f64>,
    k: f64,
}

impl EloSystem {
    fn new() -> Self {
        EloSystem { ratings: BTreeMap::new(), k: 32.0 }
    }
    fn add(&mut self, p: &str) {
        self.ratings.insert(p.to_string(), 1200.0);
    }
    fn expected(ra: f64, rb: f64) -> f64 {
        1.0 / (1.0 + 10f64.powf((rb - ra) / 400.0))
    }
    fn record_game(&mut self, w: &str, l: &str) {
        let rw = self.ratings[w];
        let rl = self.ratings[l];
        let ew = Self::expected(rw, rl);
        let delta = self.k * (1.0 - ew);
        self.ratings.insert(w.to_string(), rw + delta);
        self.ratings.insert(l.to_string(), rl - delta);
        println!("{} beats {}: {} {}->{}, {} {}->{}",
            w, l, w, rw.round() as i64, (rw + delta).round() as i64,
            l, rl.round() as i64, (rl - delta).round() as i64);
    }
}

fn main() {
    let mut e = EloSystem::new();
    for p in ["A", "B", "C", "D"] {
        e.add(p);
    }
    e.record_game("A", "B");
    e.record_game("A", "C");
    e.record_game("D", "A");
    println!("Final ratings:");
    for (p, r) in &e.ratings {
        println!("{} {}", p, r.round() as i64);
    }
}
