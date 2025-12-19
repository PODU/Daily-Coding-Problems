// Day 761: rand5() from rand7() via rejection sampling.
// Accept values 1..5, reject 6..7 and retry. Uniform; expected O(1) calls (7/5).
struct Rng {
    s: u64,
}
impl Rng {
    fn rand7(&mut self) -> i32 {
        self.s = (self.s.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        (self.s % 7) as i32 + 1 // uniform 1..7
    }
    fn rand5(&mut self) -> i32 {
        loop {
            let x = self.rand7();
            if x <= 5 {
                return x;
            }
        }
    }
}

fn main() {
    let mut rng = Rng { s: 1 };
    let n = 100000;
    let mut cnt = [0i64; 6];
    for _ in 0..n {
        cnt[rng.rand5() as usize] += 1;
    }
    println!("counts over {} samples:", n);
    for v in 1..=5 {
        println!("{}: {}", v, cnt[v]);
    }
}
