// Day 101: Goldbach. Sieve primes up to n, then the smallest prime a with n-a
// prime gives the lexicographically smallest pair. O(n log log n).
fn goldbach(n: usize) -> Option<(usize, usize)> {
    let mut composite = vec![false; n + 1];
    let mut i = 2;
    while i * i <= n {
        if !composite[i] {
            let mut j = i * i;
            while j <= n {
                composite[j] = true;
                j += i;
            }
        }
        i += 1;
    }
    for a in 2..=n / 2 {
        if !composite[a] && !composite[n - a] {
            return Some((a, n - a));
        }
    }
    None
}

fn main() {
    let (a, b) = goldbach(4).unwrap();
    println!("{} + {} = {}", a, b, a + b); // 2 + 2 = 4
}
