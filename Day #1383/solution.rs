// Weighted sampling: build cumulative prefix array, draw u in [0,1), binary search to pick. O(n) prep, O(log n) per sample.
// Seeded LCG RNG for deterministic output.
struct Lcg(u64);
impl Lcg {
    fn next_f64(&mut self) -> f64 {
        // SplitMix64-style step, take top 53 bits as float in [0,1).
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        (z >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn main() {
    let numbers = [1, 2, 3, 4];
    let probs = [0.1, 0.5, 0.2, 0.2];
    let n = numbers.len();

    let mut cum = vec![0.0f64; n];
    let mut acc = 0.0;
    for i in 0..n {
        acc += probs[i];
        cum[i] = acc;
    }

    let mut rng = Lcg(42);
    const N: usize = 100000;
    let mut counts = vec![0u64; n];
    for _ in 0..N {
        let u = rng.next_f64();
        // lower_bound on cum for first cum[i] >= u
        let mut lo = 0usize;
        let mut hi = n - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cum[mid] < u {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        counts[lo] += 1;
    }

    for i in 0..n {
        println!("{}: {:.2}", numbers[i], counts[i] as f64 / N as f64);
    }
}
