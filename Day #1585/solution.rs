// Day 1585: Sieve of Eratosthenes + incremental infinite prime generator.
// Sieve marks composites up to N; PrimeGen yields primes indefinitely via a sieve map.
// Time: O(N log log N) sieve; Space: O(N).
use std::collections::HashMap;

fn sieve(n: usize) -> Vec<usize> {
    let mut comp = vec![false; n];
    let mut primes = Vec::new();
    for i in 2..n {
        if !comp[i] {
            primes.push(i);
            let mut j = i * i;
            while j < n {
                comp[j] = true;
                j += i;
            }
        }
    }
    primes
}

struct PrimeGen {
    composites: HashMap<u64, Vec<u64>>,
    n: u64,
}

impl Iterator for PrimeGen {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            let n = self.n;
            self.n += 1;
            match self.composites.remove(&n) {
                None => {
                    self.composites.entry(n * n).or_default().push(n);
                    return Some(n);
                }
                Some(hits) => {
                    for p in hits {
                        self.composites.entry(n + p).or_default().push(p);
                    }
                }
            }
        }
    }
}

fn prime_gen() -> PrimeGen {
    PrimeGen { composites: HashMap::new(), n: 2 }
}

fn main() {
    println!("Primes < 30: {:?}", sieve(30));
    let first10: Vec<u64> = prime_gen().take(10).collect();
    println!("First 10 primes: {:?}", first10);
}
