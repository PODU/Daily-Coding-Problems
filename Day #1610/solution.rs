// Reservoir sampling (reservoir size 1): for the i-th element replace kept with prob 1/i.
// Distribution is uniform over the stream. Time O(n), Space O(1). Seeded RNG -> deterministic.

// Minimal deterministic seeded PRNG (xorshift64*).
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed } }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    // Uniform integer in [0, n).
    fn next_below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

fn reservoir_sample(stream: &[i64], seed: u64) -> i64 {
    let mut rng = Rng::new(seed);
    let mut kept = 0i64;
    for (i, &value) in stream.iter().enumerate() {
        // for the (i+1)-th element keep with prob 1/(i+1)
        if rng.next_below((i as u64) + 1) == 0 {
            kept = value;
        }
    }
    kept
}

fn main() {
    let stream: Vec<i64> = (1..=10).collect();
    let selected = reservoir_sample(&stream, 42);
    println!("Selected: {}", selected);
}
