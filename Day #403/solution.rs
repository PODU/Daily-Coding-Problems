// rand7 from rand5 via rejection sampling: idx=(rand5-1)*5+rand5 in 1..25, reject>21,
// return (idx-1)%7+1. O(1) expected calls. Space O(1).

// Deterministic LCG so the histogram is reproducible (no external crates).
struct Lcg { state: u64 }
impl Lcg {
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state >> 33
    }
    fn rand5(&mut self) -> u64 { self.next() % 5 + 1 }
    fn rand7(&mut self) -> u64 {
        loop {
            let idx = (self.rand5() - 1) * 5 + self.rand5(); // uniform 1..25
            if idx <= 21 {
                return (idx - 1) % 7 + 1;
            }
        }
    }
}

fn main() {
    let n = 70000;
    let mut rng = Lcg { state: 12345 };
    let mut counts = [0usize; 8];
    for _ in 0..n {
        counts[rng.rand7() as usize] += 1;
    }
    for v in 1..=7 {
        println!("{}: {} {}", v, counts[v], "#".repeat(counts[v] / 250));
    }
}
