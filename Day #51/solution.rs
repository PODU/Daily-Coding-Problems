// Day 51: Uniform shuffle via Fisher-Yates, using only rand(1..k) and swaps.
// Each of n! permutations equally likely. Time: O(n), Space: O(1).
use std::time::{SystemTime, UNIX_EPOCH};

// Minimal xorshift PRNG so the file is self-contained (no external deps).
struct Rng(u64);
impl Rng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Rng(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    // Perfectly random integer in [1, k].
    fn rand_k(&mut self, k: usize) -> usize {
        (self.next_u64() % k as u64) as usize + 1
    }
}

fn shuffle(deck: &mut [usize], rng: &mut Rng) {
    for i in (1..deck.len()).rev() {
        let j = rng.rand_k(i + 1) - 1; // uniform index in [0, i]
        deck.swap(i, j);
    }
}

fn main() {
    let mut rng = Rng::new();
    let mut deck: Vec<usize> = (0..52).collect();
    shuffle(&mut deck, &mut rng);
    println!(
        "{}",
        deck.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ")
    );
    let mut seen = deck.clone();
    seen.sort();
    let ok = seen.iter().enumerate().all(|(i, &v)| v == i);
    println!("valid permutation: {}", ok);
}
