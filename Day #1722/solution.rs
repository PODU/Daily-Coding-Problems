// Day 1722: Approximate median via random sampling.
// Sample a sublinear number of elements (~constant), return their exact median.
// With high probability its rank lies in [N/4, 3N/4]. Time: O(s log s) << O(N), Space: O(s).

// Deterministic xorshift PRNG (no external crates).
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

fn approx_median(a: &[i32], rng: &mut Rng) -> i32 {
    let n = a.len();
    let s = n.min(99); // sublinear sample size
    let mut sample: Vec<i32> = (0..s).map(|_| a[rng.below(n)]).collect();
    sample.sort();
    sample[s / 2]
}

fn main() {
    // Demo: values 0..99 shuffled deterministically.
    let n = 100usize;
    let mut a: Vec<i32> = (0..n as i32).collect();
    let mut rng = Rng::new(42);
    // Fisher-Yates shuffle.
    for i in (1..n).rev() {
        let j = rng.below(i + 1);
        a.swap(i, j);
    }

    let m = approx_median(&a, &mut rng);
    let rank = a.iter().filter(|&&x| x < m).count();
    println!(
        "Approximate median: {} (rank {} within [N/4, 3N/4] = [{}, {}])",
        m,
        rank,
        n / 4,
        3 * n / 4
    );
}
