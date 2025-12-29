// Sieve of Eratosthenes for primes below N; incremental generator via trial division by sqrt.
// Sieve: O(N log log N). Generator yields primes indefinitely.

fn sieve(n: usize) -> Vec<usize> {
    let mut comp = vec![false; n];
    let mut primes = Vec::new();
    let mut i = 2;
    while i < n {
        if !comp[i] {
            primes.push(i);
            let mut j = i * i;
            while j < n {
                comp[j] = true;
                j += i;
            }
        }
        i += 1;
    }
    primes
}

// Incremental generator: trial division of candidates against found primes up to sqrt.
struct PrimeGen {
    primes: Vec<u64>,
    cand: u64,
}

impl Iterator for PrimeGen {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
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
            let c = self.cand;
            self.cand += 1;
            if is_prime {
                self.primes.push(c);
                return Some(c);
            }
        }
    }
}

fn gen_primes() -> PrimeGen {
    PrimeGen { primes: Vec::new(), cand: 2 }
}

fn fmt_vec<T: ToString>(v: &[T]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    println!("Primes below 30: {}", fmt_vec(&sieve(30)));
    let first10: Vec<u64> = gen_primes().take(10).collect();
    println!("First 10 primes: {}", fmt_vec(&first10));
}
