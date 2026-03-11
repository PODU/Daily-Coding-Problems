// Approximate median: take a constant-size random sample and return its median.
// Sublinear: O(k log k) for constant k samples, independent of N. Space: O(k).
// Uses a small self-contained splitmix64 PRNG seeded with a fixed value (no external deps).

struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        // splitmix64: good bit quality, safe to take modulo.
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn range(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }
}

fn approx_median(a: &[i64]) -> (i64, usize) {
    let n = a.len();
    let k = if n < 31 { n } else { 31 };
    let mut rng = Rng::new(42); // fixed seed for reproducibility
    let mut sample: Vec<i64> = (0..k).map(|_| a[rng.range(n)]).collect();
    sample.sort();
    let med = sample[k / 2];
    let rank = a.iter().filter(|&&x| x <= med).count();
    (med, rank)
}

fn main() {
    let n = 100usize;
    let a: Vec<i64> = (1..=100).collect();
    let (med, rank) = approx_median(&a);
    println!("Approximate median: {} (rank {} within [{}, {}])", med, rank, n / 4, 3 * n / 4);
}
