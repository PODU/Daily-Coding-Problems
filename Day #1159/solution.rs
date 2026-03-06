// Goldbach: iterate a from 2 upward, return first (a, n-a) both prime (lexicographically smallest).
// Time: O(n*sqrt(n)) worst, Space: O(1).
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

fn goldbach(n: i64) -> (i64, i64) {
    let mut a = 2i64;
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
    println!("{} + {} = {}", a, b, n);
}
