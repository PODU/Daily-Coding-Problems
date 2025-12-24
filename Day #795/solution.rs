// Von Neumann debiasing: toss biased coin twice; (0,1)->0, (1,0)->1, else retry.
// Output is provably fair regardless of bias. O(1) expected tosses per fair bit.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_f64(&mut self) -> f64 { // LCG in [0,1)
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn toss_biased(rng: &mut Rng) -> u32 {
    if rng.next_f64() < 0.3 {
        1
    } else {
        0
    } // P(1) = 0.3
}

fn fair_coin(rng: &mut Rng) -> u32 {
    loop {
        let a = toss_biased(rng);
        let b = toss_biased(rng);
        if a == 0 && b == 1 {
            return 0;
        }
        if a == 1 && b == 0 {
            return 1;
        }
    }
}

fn main() {
    let mut rng = Rng::new(123456789);
    const N: u32 = 100000;
    let mut ones = 0u32;
    for _ in 0..N {
        ones += fair_coin(&mut rng);
    }
    println!("{:.1}", ones as f64 / N as f64);
}
