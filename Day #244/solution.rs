// Sieve of Eratosthenes: primes < N in O(N log log N) time, O(N) space.
// Plus an indefinite prime generator implemented as an Iterator using trial division by known primes.

fn sieve(n: usize) -> Vec<usize> {
    let mut comp = vec![false; n.max(0)];
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

// Indefinite generator as an Iterator via trial division.
struct PrimeGen {
    primes: Vec<u64>,
    cand: u64,
}

impl PrimeGen {
    fn new() -> Self {
        PrimeGen { primes: Vec::new(), cand: 1 }
    }
}

impl Iterator for PrimeGen {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            self.cand += 1;
            let mut is_prime = true;
            for &p in &self.primes {
                if p * p > self.cand {
                    break;
                }
                if self.cand % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.push(self.cand);
                return Some(self.cand);
            }
        }
    }
}

fn main() {
    let primes = sieve(100);
    let line: Vec<String> = primes.iter().map(|p| p.to_string()).collect();
    println!("{}", line.join(" "));

    let gen = PrimeGen::new();
    let first10: Vec<String> = gen.take(10).map(|p| p.to_string()).collect();
    println!("{}", first10.join(" "));
}
