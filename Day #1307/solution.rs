// Fisher-Yates shuffle using a uniform rand(1..k). Each of N! permutations
// equally likely. Time O(N), Space O(1) extra.

// Simple deterministic LCG so the demo is reproducible.
struct Rng { state: u64 }
impl Rng {
    fn new(seed: u64) -> Self { Rng { state: seed } }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state >> 33
    }
    // uniform random integer in [1, k]
    fn rand_k(&mut self, k: u64) -> u64 { (self.next() % k) + 1 }
}

fn shuffle_deck(deck: &mut [usize], rng: &mut Rng) {
    let n = deck.len();
    for i in (1..n).rev() {
        let j = (rng.rand_k((i + 1) as u64) - 1) as usize; // uniform in [0, i]
        deck.swap(i, j);
    }
}

fn main() {
    let mut rng = Rng::new(12345);
    let mut deck: Vec<usize> = (1..=52).collect();
    shuffle_deck(&mut deck, &mut rng);
    let s: Vec<String> = deck.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}
