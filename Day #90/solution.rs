// Day 90: Uniform random in [0,n) excluding l. Pick r in [0, n-k), then offset past
// sorted exclusions so every valid value is equally likely. Time O(k log k), Space O(k).
// Self-contained xorshift RNG to avoid external crates.
use std::collections::BTreeSet;

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next_u64() % m
    }
}

fn random_excluding(n: i64, l: &[i64], rng: &mut Rng) -> i64 {
    let ex: BTreeSet<i64> = l.iter().cloned().filter(|&v| v >= 0 && v < n).collect();
    let m = n - ex.len() as i64;
    if m <= 0 {
        panic!("no valid number");
    }
    let mut r = rng.below(m as u64) as i64;
    for &e in &ex {
        if e <= r {
            r += 1;
        } else {
            break;
        }
    }
    r
}

fn main() {
    let mut rng = Rng(42);
    println!("{}", random_excluding(5, &[1, 3], &mut rng)); // sample from {0,2,4}
}
