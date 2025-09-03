// Day 210: Collatz conjecture - verify reach to 1 and find longest sequence for n <= 1e6.
// Memoized sequence lengths (cache for values <= LIMIT). Time: ~O(LIMIT * avg steps), Space: O(LIMIT).
const LIMIT: usize = 1_000_000;

fn collatz_len(start: u64, cache: &mut [i32]) -> i32 {
    let mut path: Vec<u64> = Vec::new();
    let mut m = start;
    while m > LIMIT as u64 || cache[m as usize] == 0 {
        path.push(m);
        m = if m % 2 == 0 { m / 2 } else { 3 * m + 1 };
    }
    let mut base = cache[m as usize];
    for &v in path.iter().rev() {
        base += 1;
        if v <= LIMIT as u64 {
            cache[v as usize] = base;
        }
    }
    base
}

fn main() {
    let mut cache = vec![0i32; LIMIT + 1];
    cache[1] = 1;
    println!("Collatz length of 27: {}", collatz_len(27, &mut cache)); // 112
    let (mut best_n, mut best_len) = (1usize, 1i32);
    for n in 1..=LIMIT {
        let l = collatz_len(n as u64, &mut cache);
        if l > best_len {
            best_len = l;
            best_n = n;
        }
    }
    println!(
        "Longest sequence for n <= 1000000: n={} (length {})",
        best_n, best_len
    ); // n=837799 (length 525)
}
