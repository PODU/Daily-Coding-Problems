// rand5 from rand7 via rejection sampling: keep rand7 values in 1..5. Expected O(1) calls (7/5).

// Self-contained xorshift PRNG for a reproducible demo.
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
    fn rand7(&mut self) -> i32 {
        (self.next() % 7) as i32 + 1
    }
    fn rand5(&mut self) -> i32 {
        let mut x = self.rand7();
        while x > 5 {
            x = self.rand7();
        }
        x
    }
}

fn main() {
    let mut rng = Rng(12345);
    let mut counts = [0i32; 6];
    let trials = 100000;
    for _ in 0..trials {
        counts[rng.rand5() as usize] += 1;
    }
    println!("Distribution over {} samples:", trials);
    for v in 1..=5 {
        println!("{}: {}", v, counts[v]);
    }
}
