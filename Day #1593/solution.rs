// rand5 from rand7 via rejection sampling: draw rand7, accept if <=5 else retry.
// Expected O(1) calls (acceptance prob 5/7). Output uniform on 1..5.

// Deterministic seeded PRNG (xorshift64) so output is reproducible (no external deps).
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
    // uniform 1..7
    fn rand7(&mut self) -> u32 {
        (self.next_u64() % 7) as u32 + 1
    }
    // uniform 1..5 via rejection sampling
    fn rand5(&mut self) -> u32 {
        loop {
            let v = self.rand7();
            if v <= 5 {
                return v;
            }
        }
    }
}

fn main() {
    let mut rng = Rng::new(12345);
    const N: usize = 100000;
    let mut counts = [0usize; 6];
    for _ in 0..N {
        counts[rng.rand5() as usize] += 1;
    }

    println!("Distribution over {} samples:", N);
    for v in 1..=5 {
        println!("  {}: {}", v, counts[v]);
    }

    let parts: Vec<String> = (0..10).map(|_| rng.rand5().to_string()).collect();
    println!("First 10 samples: {}", parts.join(" "));
}
