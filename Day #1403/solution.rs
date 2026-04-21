// Monte-Carlo simulation of two stop conditions plus exact theory.
// E[rolls until 5->6] = 36 (distinct faces); E[rolls until 5->5] = 42 (same face).
// Time O(trials * rolls), Space O(1).

// Simple xorshift64 PRNG for reproducibility without external crates.
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
    fn roll(&mut self) -> i32 {
        (self.next() % 6) as i32 + 1
    }
}

fn play(second: i32, rng: &mut Rng) -> i64 {
    let mut rolls = 0i64;
    let mut prev = 0;
    loop {
        let cur = rng.roll();
        rolls += 1;
        if prev == 5 && cur == second {
            return rolls;
        }
        prev = cur;
    }
}

fn main() {
    let mut rng = Rng(42);
    let trials: i64 = 200000;
    let mut s56 = 0i64;
    let mut s55 = 0i64;
    for _ in 0..trials {
        s56 += play(6, &mut rng);
    }
    for _ in 0..trials {
        s55 += play(5, &mut rng);
    }
    println!(
        "Game 1 (five then six): simulated avg = {:.2}, theoretical = 36",
        s56 as f64 / trials as f64
    );
    println!(
        "Game 2 (five then five): simulated avg = {:.2}, theoretical = 42",
        s55 as f64 / trials as f64
    );
    println!("Alice should play Game 1 (five-then-six): fewer expected rolls, less pay.");
}
