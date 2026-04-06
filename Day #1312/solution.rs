// rand7 from rand5 via rejection sampling: combine two rand5 into uniform
// 1..25, accept 1..21, map to 1..7. Expected O(1) calls, O(1) space.

struct Rng { state: u64 }
impl Rng {
    fn new(seed: u64) -> Self { Rng { state: seed } }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state >> 33
    }
    fn rand5(&mut self) -> i32 { (self.next() % 5) as i32 + 1 } // uniform 1..5
    fn rand7(&mut self) -> i32 {
        loop {
            let v = (self.rand5() - 1) * 5 + (self.rand5() - 1); // uniform 0..24
            if v < 21 {
                return v % 7 + 1; // accept 0..20 -> 1..7
            }
        }
    }
}

fn main() {
    let mut rng = Rng::new(42);
    let mut counts = [0i32; 8];
    for _ in 0..70000 {
        counts[rng.rand7() as usize] += 1;
    }
    for i in 1..=7 {
        println!("{}: {}", i, counts[i]);
    }
}
