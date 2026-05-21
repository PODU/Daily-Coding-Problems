// Fisher-Yates shuffle: uniform random permutation using only swaps.
// rand(k) gives a uniform int in [1,k]; each of N! orderings is equally likely.
// Time O(N), Space O(1). Uses a small deterministic LCG PRNG.
struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }
    // uniform random number between 1 and k inclusive
    fn randk(&mut self, k: u64) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 33) % k + 1
    }
}

fn shuffle(deck: &mut [u32], rng: &mut Lcg) {
    let n = deck.len();
    for i in (1..n).rev() {
        let j = (rng.randk((i + 1) as u64) - 1) as usize; // index in [0, i]
        deck.swap(i, j);
    }
}

fn main() {
    let mut rng = Lcg::new(12345);
    let mut deck: Vec<u32> = (1..=52).collect();
    shuffle(&mut deck, &mut rng);
    let s: Vec<String> = deck.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}
