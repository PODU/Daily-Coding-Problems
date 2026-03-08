// Day 1178: Collatz sequences. Verify each reaches 1 and find the longest start <= 1e6.
// Memoized chain lengths (each value cached once). Time ~O(LIMIT), Space O(LIMIT).

const LIMIT: i64 = 1_000_000;

fn clen(n: i64, memo: &mut Vec<i32>) -> i32 {
    if n == 1 {
        return 1;
    }
    if n <= LIMIT && memo[n as usize] != 0 {
        return memo[n as usize];
    }
    let nxt = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    let l = 1 + clen(nxt, memo);
    if n <= LIMIT {
        memo[n as usize] = l;
    }
    l
}

fn main() {
    let mut memo = vec![0i32; (LIMIT + 1) as usize];

    let mut seq = Vec::new();
    let mut n: i64 = 6;
    loop {
        seq.push(n.to_string());
        if n == 1 {
            break;
        }
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }
    println!("{}", seq.join(" -> "));

    let (mut best_n, mut best_len) = (1i64, 1i32);
    for i in 2..=LIMIT {
        let l = clen(i, &mut memo);
        if l > best_len {
            best_len = l;
            best_n = i;
        }
    }
    println!("All sequences up to {} reach 1: true", LIMIT);
    println!("Longest sequence for n <= {}: n = {} (length {})", LIMIT, best_n, best_len);
}
