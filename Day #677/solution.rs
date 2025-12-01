// Sieve of Eratosthenes for primes < N (O(N log log N)); plus an incremental
// generator using a map of next composite multiples. Space: O(N) sieve.
use std::collections::HashMap;

fn sieve(n: usize) -> Vec<usize> {
    let mut is_comp = vec![false; n];
    let mut primes = Vec::new();
    for i in 2..n {
        if !is_comp[i] {
            primes.push(i);
            let mut j = i * i;
            while j < n {
                is_comp[j] = true;
                j += i;
            }
        }
    }
    primes
}

// Incremental generator: yields primes one-by-one via a map of next composites.
struct PrimeGen {
    composites: HashMap<u64, u64>,
    current: u64,
}

impl PrimeGen {
    fn new() -> Self {
        PrimeGen { composites: HashMap::new(), current: 1 }
    }
    fn next(&mut self) -> u64 {
        loop {
            self.current += 1;
            if let Some(prime) = self.composites.remove(&self.current) {
                let mut x = self.current + prime;
                while self.composites.contains_key(&x) {
                    x += prime;
                }
                self.composites.insert(x, prime);
            } else {
                self.composites.insert(self.current * self.current, self.current);
                return self.current;
            }
        }
    }
}

fn main() {
    let primes: Vec<String> = sieve(100).iter().map(|p| p.to_string()).collect();
    println!("[{}]", primes.join(", "));

    let mut gen = PrimeGen::new();
    let first10: Vec<String> = (0..10).map(|_| gen.next().to_string()).collect();
    println!("[{}]", first10.join(", "));
}
