// Von Neumann fair coin from a biased toss: toss twice, (0,1)->0, (1,0)->1, else retry. O(1) expected per fair toss.

struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    // xorshift64* -> uniform f64 in [0,1)
    fn next_f64(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        let val = x.wrapping_mul(0x2545F4914F6CDD1D);
        (val >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn toss_biased(rng: &mut Rng) -> i32 {
    if rng.next_f64() < 0.3 { 1 } else { 0 }
}

fn fair_toss(rng: &mut Rng) -> i32 {
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
    let mut rng = Rng::new(12345);
    let n = 100000;
    let mut heads = 0;
    for _ in 0..n {
        heads += fair_toss(&mut rng);
    }
    println!("heads fraction: {:.2}", heads as f64 / n as f64);
}
