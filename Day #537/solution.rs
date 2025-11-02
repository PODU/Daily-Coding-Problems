// Collatz: iterate n->n/2 (even) or 3n+1 (odd) counting steps to 1; verify a range.
// Bonus longest under 1e6 via memoized step counts. Time ~O(LIMIT*avg-chain), space O(LIMIT).

fn steps(mut n: i64) -> i32 {
    let mut c = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        c += 1;
    }
    c
}

fn main() {
    let mut all_reach = true;
    for n in 1..=20 {
        if steps(n) < 0 {
            all_reach = false;
        }
    }
    println!("Collatz conjecture holds for 1..20: {}", all_reach);

    const LIMIT: usize = 1_000_000;
    let mut dp = vec![0i32; LIMIT + 1];
    let (mut best_n, mut best_len) = (1usize, 0i32);
    for i in 2..=LIMIT {
        let mut n = i as i64;
        let mut c = 0;
        while n != 1 && (n as usize > LIMIT || dp[n as usize] == 0) {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            c += 1;
        }
        if n != 1 {
            c += dp[n as usize];
        }
        dp[i] = c;
        if c > best_len {
            best_len = c;
            best_n = i;
        }
    }
    println!("Longest under 1000000: n={} with {} steps", best_n, best_len);
}
