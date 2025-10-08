// Presence bitset: mark each present value, then report unmarked ones.
// Time: O(N), Space: O(N) bits.  (N = 1,000,000)

fn find_missing(present: &[usize], n: usize) -> Vec<usize> {
    let mut seen = vec![false; n + 1];
    for &x in present {
        seen[x] = true;
    }
    (1..=n).filter(|&i| !seen[i]).collect()
}

fn main() {
    let n = 1_000_000usize;
    let present: Vec<usize> = (1..=n).filter(|i| i % 1000 != 0).collect();

    let missing = find_missing(&present, n);
    println!("Missing count: {}", missing.len());
    let first10: Vec<String> = missing.iter().take(10).map(|v| v.to_string()).collect();
    println!("First 10 missing: {}", first10.join(" "));
    println!("Time complexity: O(N), Space complexity: O(N) bits (N = 1,000,000)");
}
