// Fisher-Yates shuffle of linked-list node values via vector: O(n) time, O(n) space.
// Space-over-time tradeoff: O(1)-extra approach repeatedly picks a random node by traversal in O(n^2) time.

// simple deterministic LCG for reproducible demo (no external deps)
struct Rng { state: u64 }
impl Rng {
    fn new(seed: u64) -> Self { Rng { state: seed } }
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }
    fn below(&mut self, n: usize) -> usize { (self.next_u64() % (n as u64)) as usize }
}

fn main() {
    // represent nodes as values in a vec (Box-linked list is awkward for shuffle in safe Rust)
    let mut a: Vec<i32> = (1..=5).collect();
    let n = a.len();
    let mut rng = Rng::new(12345);
    for i in (1..n).rev() {
        let j = rng.below(i + 1);
        a.swap(i, j);
    }
    // a is now the shuffled order; verify valid permutation
    let orig: Vec<i32> = (1..=5).collect();
    let mut shuf = a.clone();
    shuf.sort();
    let valid = shuf == orig;
    let parts: Vec<String> = orig.iter().map(|v| v.to_string()).collect();
    let msg = if valid { "valid shuffle (same elements)" } else { "INVALID" };
    println!("{} -> {}", parts.join(" "), msg);
}
