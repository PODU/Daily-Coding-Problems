// Reservoir sampling (k=1): keep current pick with prob 1/i as i-th item streams.
// Time: O(n) single pass, Space: O(1). Uses a small seeded xorshift PRNG.
use std::collections::BTreeMap;

struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

fn pick_random(stream: &[i32], rng: &mut Rng) -> i32 {
    let mut chosen = 0;
    let mut count: u64 = 0;
    for &x in stream {
        count += 1;
        if rng.below(count) == 0 {
            chosen = x;
        }
    }
    chosen
}

fn main() {
    let mut rng = Rng::new(88172645463325252);
    let stream = [10, 20, 30, 40, 50];
    let mut freq: BTreeMap<i32, i32> = BTreeMap::new();
    for _ in 0..100000 {
        *freq.entry(pick_random(&stream, &mut rng)).or_insert(0) += 1;
    }
    println!("One sample: {}", pick_random(&stream, &mut rng));
    println!("Approx frequencies over 100000 trials:");
    for (k, v) in &freq {
        println!("  {}: {:.3}", k, *v as f64 / 100000.0);
    }
}
