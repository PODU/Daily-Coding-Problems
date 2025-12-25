// Day 797: Goldbach - two primes summing to even n, lexicographically smallest.
// Scan a from 2 upward; first prime a with prime (n-a) gives smallest pair.
// Time O(n * sqrt(n)), Space O(1).

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn goldbach(n: i64) -> (i64, i64) {
    let mut a = 2;
    while a <= n / 2 {
        if is_prime(a) && is_prime(n - a) {
            return (a, n - a);
        }
        a += 1;
    }
    (-1, -1)
}

fn main() {
    let n = 4;
    let (a, b) = goldbach(n);
    println!("{} + {} = {}", a, b, n); // 2 + 2 = 4
}
