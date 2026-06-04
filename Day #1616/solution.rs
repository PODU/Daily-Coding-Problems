// Goldbach pair: sieve up to n, scan a from 2; first a where a and n-a prime.
// Smallest a => lexicographically smallest [a,b]. Time O(n log log n), Space O(n).
fn goldbach(n: usize) -> (usize, usize) {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut a = 2;
    while a <= n - a {
        if is_prime[a] && is_prime[n - a] {
            return (a, n - a);
        }
        a += 1;
    }
    (0, 0)
}

fn main() {
    let n = 4;
    let (a, b) = goldbach(n);
    println!("{} + {} = {}", a, b, n);
}
