// Monte Carlo pi: sample (x,y) in unit square via deterministic splitmix64 RNG,
// pi ~= 4*inside/total. Fixed seed -> deterministic. Time O(N), Space O(1).

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn next_double(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

fn main() {
    let mut rng = SplitMix64 { state: 42 };
    const N: u64 = 10_000_000;
    let mut inside: u64 = 0;
    for _ in 0..N {
        let x = rng.next_double();
        let y = rng.next_double();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    let pi = 4.0 * inside as f64 / N as f64;
    println!("Estimated pi \u{2248} {:.3}", pi);
}
