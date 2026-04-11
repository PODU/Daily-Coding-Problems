// Sieve of Eratosthenes: mark multiples of each prime from p*p. O(N log log N) time, O(N) space.
// Bonus: an incremental-sieve Iterator yielding primes indefinitely.
use std::collections::HashMap;

fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut composite = vec![false; n];
    let mut primes = Vec::new();
    for p in 2..n {
        if !composite[p] {
            primes.push(p);
            let mut m = p * p;
            while m < n {
                composite[m] = true;
                m += p;
            }
        }
    }
    primes
}

struct PrimeGenerator {
    composites: HashMap<u64, Vec<u64>>,
    candidate: u64,
}

impl PrimeGenerator {
    fn new() -> Self {
        PrimeGenerator { composites: HashMap::new(), candidate: 1 }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            self.candidate += 1;
            if let Some(factors) = self.composites.remove(&self.candidate) {
                for p in factors {
                    self.composites.entry(self.candidate + p).or_default().push(p);
                }
            } else {
                self.composites.insert(self.candidate * self.candidate, vec![self.candidate]);
                return Some(self.candidate);
            }
        }
    }
}

fn main() {
    let primes: Vec<String> = sieve(100).iter().map(|p| p.to_string()).collect();
    println!("{}", primes.join(" "));

    let gen = PrimeGenerator::new();
    let first10: Vec<String> = gen.take(10).map(|p| p.to_string()).collect();
    println!("{}", first10.join(" "));
}
