// Uniform random in [0,n-1] excluding values in l: precompute sorted allowed array, pick random index. O(n) build, O(allowed) space.
use std::collections::HashSet;

// Tiny deterministic LCG so the demo needs no external crates.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
}

struct RandomPicker {
    allowed: Vec<i32>,
    rng: Lcg,
}
impl RandomPicker {
    fn new(n: i32, l: &[i32]) -> Self {
        let ex: HashSet<i32> = l.iter().cloned().collect();
        let allowed: Vec<i32> = (0..n).filter(|i| !ex.contains(i)).collect();
        RandomPicker { allowed, rng: Lcg(12345) }
    }
    fn pick(&mut self) -> i32 {
        let idx = (self.rng.next() % self.allowed.len() as u64) as usize;
        self.allowed[idx]
    }
}

fn main() {
    let mut rp = RandomPicker::new(10, &[2, 3, 5, 7]);
    let aset: HashSet<i32> = rp.allowed.iter().cloned().collect();
    let parts: Vec<String> = rp.allowed.iter().map(|v| v.to_string()).collect();
    println!("allowed=[{}]", parts.join(", "));
    let mut ok = true;
    for _ in 0..1000 {
        if !aset.contains(&rp.pick()) {
            ok = false;
        }
    }
    println!("sample in allowed: {}", ok);
}
