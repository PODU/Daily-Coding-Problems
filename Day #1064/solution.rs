// Day 1064: Implement rand7() from rand5() via rejection sampling.
// (rand5()-1)*5 + rand5() -> uniform 1..25; reject >21; ((v-1)%7)+1. Expected O(1) calls, O(1) space.

// Deterministic xorshift64 PRNG so the demo is reproducible without external deps.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    fn rand5(&mut self) -> u64 {
        self.next_u64() % 5 + 1
    }
    fn rand7(&mut self) -> u64 {
        loop {
            let v = (self.rand5() - 1) * 5 + self.rand5(); // uniform 1..25
            if v <= 21 {
                return (v - 1) % 7 + 1;
            }
        }
    }
}

fn main() {
    let mut rng = Rng::new(42);
    let mut counts = [0u32; 8];
    for _ in 0..70000 {
        counts[rng.rand7() as usize] += 1;
    }
    for i in 1..=7 {
        println!("{}: {}", i, counts[i]);
    }
}
