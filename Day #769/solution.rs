// Day 769: Expected rolls for "5 then 6" vs "5 then 5" stopping games.
// Exact via 2-state Markov chains (E1=36, E2=42) plus Monte-Carlo check.
// Self-contained xorshift RNG (no external crates).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn die(&mut self) -> i32 {
        (self.next() % 6) as i32 + 1
    }
}

fn simulate(second: i32, trials: u64, rng: &mut Rng) -> f64 {
    let mut total: u64 = 0;
    for _ in 0..trials {
        let mut prev = 0;
        let mut rolls = 0u64;
        loop {
            let r = rng.die();
            rolls += 1;
            if prev == 5 && r == second {
                break;
            }
            prev = r;
        }
        total += rolls;
    }
    total as f64 / trials as f64
}

fn main() {
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let trials = 300000;
    println!("Game 1 (5 then 6): exact=36, simulated={:.2}", simulate(6, trials, &mut rng));
    println!("Game 2 (5 then 5): exact=42, simulated={:.2}", simulate(5, trials, &mut rng));
    println!("Alice should play Game 1 (5 then 6); it has the lower expected cost.");
}
