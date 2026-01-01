// Day 836: Fisher-Yates shuffle using only a rand(k) RNG (uniform 1..k) and swaps.
// For i=n-1..1: pick j uniform in 0..i via rand(i+1)-1, swap a[i],a[j]. O(N) time, O(1) extra.
// Unbiased: each step picks uniformly among i+1 positions, so all n! permutations are equally likely.

struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state >> 16
    }
    // Uniform in [1, k] with rejection to avoid modulo bias.
    fn rand(&mut self, k: u64) -> u64 {
        let mask: u64 = (1u64 << 48) - 1;
        let limit: u64 = (1u64 << 48) - ((1u64 << 48) % k);
        loop {
            let r = self.next() & mask;
            if r < limit {
                return r % k + 1;
            }
        }
    }
}

fn shuffle(a: &mut Vec<i32>, rng: &mut Rng) {
    for i in (1..a.len()).rev() {
        let j = (rng.rand((i + 1) as u64) - 1) as usize; // uniform 0..i
        a.swap(i, j);
    }
}

fn main() {
    let mut deck: Vec<i32> = (1..=52).collect();
    let mut rng = Rng::new(12345);
    shuffle(&mut deck, &mut rng);
    let parts: Vec<String> = deck.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
