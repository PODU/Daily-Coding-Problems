// Approximate median: take a small random sample (size s) and return its median.
// Rank lands in [N/4, 3N/4] w.h.p. Time O(s log s) = sub-linear, Space O(s).

// Small xorshift PRNG for a self-contained, reproducible demo.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn approx_median(a: &[i32], sample_size: usize, rng: &mut Rng) -> i32 {
    let n = a.len();
    let s = sample_size.min(n);
    let mut sample: Vec<i32> = (0..s).map(|_| a[rng.below(n)]).collect();
    sample.sort_unstable();
    sample[s / 2]
}

fn main() {
    let mut rng = Rng(42);
    let n = 1000usize;
    let mut a: Vec<i32> = (0..n as i32).collect();
    // Fisher-Yates shuffle.
    for i in (1..n).rev() {
        let j = rng.below(i + 1);
        a.swap(i, j);
    }
    let m = approx_median(&a, 51, &mut rng);
    let rank = a.iter().filter(|&&x| x < m).count();
    println!("approx median = {}", m);
    let ok = rank >= n / 4 && rank <= 3 * n / 4;
    println!("rank {} in [{}, {}]: {}", rank, n / 4, 3 * n / 4, ok);
}
