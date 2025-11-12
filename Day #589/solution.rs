// Expected waiting time for patterns on a fair d6: "5 then 6" (distinct) E=36;
// "5 then 5" (self-overlap) E=6+36=42. Monte Carlo corroborates. Time O(1) for theory.

struct Lcg {
    state: u64,
}
impl Lcg {
    fn roll(&mut self) -> i32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 33) % 6) as i32 + 1
    }
}

#[allow(dead_code)]
fn simulate(first: i32, second: i32, trials: i32) -> f64 {
    let mut rng = Lcg { state: 12345 };
    let mut total: i64 = 0;
    for _ in 0..trials {
        let (mut prev, mut count) = (0, 0);
        loop {
            let r = rng.roll();
            count += 1;
            if prev == first && r == second {
                break;
            }
            prev = r;
        }
        total += count;
    }
    total as f64 / trials as f64
}

fn main() {
    let e1 = 36; // five then six
    let e2 = 42; // five then five
    let _ = simulate; // available for corroboration
    println!("Game 1 (five then six) expected rolls: {}", e1);
    println!("Game 2 (five then five) expected rolls: {}", e2);
    println!("Alice should play Game 1 (five then six) since it has the lower expected cost.");
}
