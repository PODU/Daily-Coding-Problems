// Approximate median: median of k random samples (seeded LCG) -> rank in [N/4, 3N/4] whp.
// Time: O(k log k), o(N) for k<N; Space: O(k).

// Simple deterministic LCG (no external deps).
struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }
    fn next_u32(&mut self) -> u32 {
        // Numerical Recipes LCG constants.
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 33) as u32
    }
    fn next_below(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }
}

fn approx_median(a: &[i32], k: usize, seed: u64) -> i32 {
    let mut rng = Lcg::new(seed);
    let mut sample: Vec<i32> = (0..k).map(|_| a[rng.next_below(a.len() as u32) as usize]).collect();
    sample.sort();
    sample[k / 2]
}

fn main() {
    let a: Vec<i32> = (0..=100).collect(); // N = 101, values 0..100
    let n = a.len() as i32;
    let val = approx_median(&a, 15, 42);
    let rank = val; // rank in sorted 0..100 equals value
    let ok = n / 4 <= rank && rank <= (3 * n) / 4;
    println!("Approximate median is within [N/4, 3N/4]: {}", if ok { "true" } else { "false" });
}
