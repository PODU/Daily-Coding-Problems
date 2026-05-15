// Two dice games via Monte Carlo simulation. Expected rolls: "5 then 6"=36, "5 then 5"=42.
// Time: O(trials * rolls_per_trial). Space: O(1).

// Simple seeded LCG so we have no external deps and reproducible output.
struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }
    fn die(&mut self) -> i32 {
        // 64-bit LCG constants (Knuth MMIX).
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 33) % 6) as i32 + 1
    }
}

fn simulate(first: i32, second: i32, trials: u64, rng: &mut Lcg) -> f64 {
    let mut total: u64 = 0;
    for _ in 0..trials {
        let mut prev = 0;
        let mut rolls = 0u64;
        loop {
            let r = rng.die();
            rolls += 1;
            if prev == first && r == second {
                break;
            }
            prev = r;
        }
        total += rolls;
    }
    total as f64 / trials as f64
}

fn main() {
    let mut rng = Lcg::new(12345);
    let trials = 500000u64;
    let e1 = simulate(5, 6, trials, &mut rng);
    let e2 = simulate(5, 5, trials, &mut rng);
    println!("Game 1 (five then six) expected rolls: {:.2}", e1);
    println!("Game 2 (five then five) expected rolls: {:.2}", e2);
    println!("Alice should play: Game 1 (five then six)");
}
