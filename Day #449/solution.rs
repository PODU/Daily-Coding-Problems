// Day 449: Goldbach pair. Scan a from 2 upward; the first a where a and n-a are
// both prime gives the lexicographically smallest pair. O(n*sqrt(n)) worst case.

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    let mut i = 2i64;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn goldbach(n: i64) -> Option<(i64, i64)> {
    let mut a = 2i64;
    while a <= n / 2 {
        if is_prime(a) && is_prime(n - a) {
            return Some((a, n - a));
        }
        a += 1;
    }
    None
}

fn main() {
    let n = 4;
    let (a, b) = goldbach(n).unwrap();
    println!("{} + {} = {}", a, b, n); // 2 + 2 = 4
}
