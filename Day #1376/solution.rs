// Uniform linked-list shuffle via Fisher-Yates. Time O(n), Space O(n) for the
// index pass (space-over-time variant: O(1) space, O(n^2) by random node selection).
// A deterministic LCG is used so output is reproducible. Uses a Vec-backed list.

struct Lcg {
    seed: u64,
}
impl Lcg {
    fn next(&mut self) -> u64 {
        self.seed = (self.seed.wrapping_mul(1103515245).wrapping_add(12345)) % 2147483648;
        self.seed
    }
}

fn shuffle(vals: &mut Vec<i32>) {
    let mut rng = Lcg { seed: 42 };
    let n = vals.len();
    for i in (1..n).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        vals.swap(i, j);
    }
}

fn main() {
    // Linked list of 1..=5 represented as a Vec (values carry the node identity).
    let mut vals: Vec<i32> = (1..=5).collect();
    shuffle(&mut vals);
    let out: Vec<String> = vals.iter().map(|v| v.to_string()).collect();
    println!("{}", out.join(" -> "));
}
