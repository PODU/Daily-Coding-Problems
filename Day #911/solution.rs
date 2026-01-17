// Reservoir sampling (size 1): replace pick with prob 1/i for i-th element. O(n) time, O(1) space.
// Self-contained xorshift RNG (no external deps).

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Rng(seed | 1) }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    // uniform in [0, n)
    fn below(&mut self, n: u64) -> u64 { self.next_u64() % n }
}

fn reservoir_sample(stream: &[i32], rng: &mut Rng) -> i32 {
    let mut pick = 0;
    for (idx, &x) in stream.iter().enumerate() {
        let i = (idx as u64) + 1; // 1-indexed
        if rng.below(i) == 0 {     // prob 1/i
            pick = x;
        }
    }
    pick
}

fn main() {
    let mut rng = Rng::new(12345);
    let n = 10usize;
    let stream: Vec<i32> = (0..n as i32).collect(); // 0..9

    println!("Sampled element: {}", reservoir_sample(&stream, &mut rng));

    let trials = 100_000;
    let mut freq = vec![0u32; n];
    for _ in 0..trials {
        let s = reservoir_sample(&stream, &mut rng) as usize;
        freq[s] += 1;
    }
    println!("Approx frequencies over {} trials (expect ~{:.4} each):", trials, 1.0 / n as f64);
    for v in 0..n {
        println!("  {}: {:.4}", v, freq[v] as f64 / trials as f64);
    }
    println!("Distribution is ~uniform.");
}
