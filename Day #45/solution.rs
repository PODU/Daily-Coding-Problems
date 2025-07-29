// rand7 from rand5: rejection sampling over 5*(rand5-1)+rand5 in 1..25,
// reject >21, map ((v-1)%7)+1. Expected O(1) amortized. rand5 from a seeded LCG.

struct Rng {
    state: i64, // deterministic seed
}

impl Rng {
    fn rand5(&mut self) -> i64 {
        self.state = (self.state * 75 + 74) % 65537;
        self.state % 5 + 1 // uniform-ish 1..5 for the demo
    }
    fn rand7(&mut self) -> i64 {
        loop {
            let v = 5 * (self.rand5() - 1) + self.rand5(); // 1..25
            if v <= 21 {
                return (v - 1) % 7 + 1;
            }
        }
    }
}

fn main() {
    let mut rng = Rng { state: 1 };

    let samples: Vec<String> = (0..20).map(|_| rng.rand7().to_string()).collect();
    println!("rand7 samples: {}", samples.join(" "));

    let mut counts = [0i64; 8];
    for _ in 0..7000 {
        counts[rng.rand7() as usize] += 1;
    }
    let parts: Vec<String> = (1..=7).map(|v| format!("{}:{}", v, counts[v])).collect();
    println!("counts over 7000 trials: {}", parts.join(" "));
}
