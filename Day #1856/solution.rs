// Day 1856: Collatz conjecture test + longest chain under 1,000,000.
// Memoized step counts (cache values <= LIMIT). ~O(LIMIT * avg-chain) time, O(LIMIT) space.

fn main() {
    const LIMIT: usize = 1_000_000;
    let mut steps = vec![0i32; LIMIT + 1]; // steps[n] = steps to reach 1 (0 = unknown; steps[1]=0)
    let all_reach_1 = true;
    let mut best_n = 1usize;
    let mut best_steps = 0i32;

    for i in 2..=LIMIT {
        let mut n: u64 = i as u64;
        let mut cnt = 0i32;
        while n != 1 && (n > LIMIT as u64 || steps[n as usize] == 0) {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            cnt += 1;
        }
        let total = cnt + if n == 1 { 0 } else { steps[n as usize] };
        steps[i] = total;
        if total > best_steps {
            best_steps = total;
            best_n = i;
        }
    }

    println!("All sequences for n in [1, {}] reach 1: {}", LIMIT, all_reach_1);
    println!(
        "Longest sequence under {}: n = {} with {} terms",
        LIMIT,
        best_n,
        best_steps + 1
    ); // 837799, 525 terms
}
