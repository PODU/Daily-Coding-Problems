// Day 1850: rand7() from rand5() via rejection sampling on the 1..25 grid.
// Expected O(1) amortized calls (acceptance 21/25); uniform over 1..7.
// Self-contained xorshift RNG to avoid external crates.

struct Rng {
    state: u64,
}
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

fn rand5(rng: &mut Rng) -> i64 {
    (rng.next_u64() % 5) as i64 + 1
}

fn rand7(rng: &mut Rng) -> i64 {
    loop {
        let v = 5 * (rand5(rng) - 1) + rand5(rng); // 1..25
        if v <= 21 {
            return (v - 1) % 7 + 1;
        }
    }
}

fn main() {
    let mut rng = Rng { state: 0x9E3779B97F4A7C15 };
    let mut counts = [0u32; 8];
    for _ in 0..70000 {
        counts[rand7(&mut rng) as usize] += 1;
    }
    let sample: Vec<String> = (0..10).map(|_| rand7(&mut rng).to_string()).collect();
    println!("Sample of 10: {}", sample.join(" "));
    println!("Histogram over 70000 draws (each ~10000):");
    for i in 1..=7 {
        println!("  {}: {}", i, counts[i]);
    }
}
