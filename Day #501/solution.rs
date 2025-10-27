// Fisher-Yates shuffle using a rand(k) helper returning [1,k]; O(N) time, O(1) extra space.
// Each of the N! permutations is equally likely. Fixed seed -> reproducible output.

// Deterministic seeded PRNG (xorshift64) so output is stable.
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
    // Uniform integer in [1, k].
    fn randk(&mut self, k: usize) -> usize {
        (self.next_u64() % (k as u64)) as usize + 1
    }
}

fn shuffle_deck(arr: &mut Vec<usize>, rng: &mut Rng) {
    let n = arr.len();
    for i in (1..n).rev() {
        let j = rng.randk(i + 1) - 1; // index in [0, i]
        arr.swap(i, j);
    }
}

fn main() {
    let mut rng = Rng::new(12345);
    let mut deck: Vec<usize> = (1..=52).collect(); // cards 1..52
    shuffle_deck(&mut deck, &mut rng);
    let parts: Vec<String> = deck.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
