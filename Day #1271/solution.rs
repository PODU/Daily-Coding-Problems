// Day 1271: Implement rand5() from rand7() with uniform probability.
// Rejection sampling: redraw rand7 until result <= 5. Expected O(7/5) calls per sample.
// Self-contained xorshift RNG so the file needs no external crates.
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
    fn rand7(&mut self) -> u32 {
        (self.next() % 7) as u32 + 1
    }
    fn rand5(&mut self) -> u32 {
        let mut v = self.rand7();
        while v > 5 {
            v = self.rand7();
        }
        v
    }
}

fn main() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let trials = 100000;
    let mut count = [0u32; 6];
    for _ in 0..trials {
        count[rng.rand5() as usize] += 1;
    }
    println!(
        "Distribution over {} samples (expect ~{} each):",
        trials,
        trials / 5
    );
    for v in 1..=5 {
        println!("{}: {}", v, count[v]);
    }
}
