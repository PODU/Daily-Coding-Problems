// Day 1296: Smallest positive integer not expressible as a subset sum of a sorted array.
// Greedy: track reachable range [1..r]; if next a[i] <= r+1 extend, else answer r+1. O(N) time.
fn smallest_non_subset_sum(a: &[i64]) -> i64 {
    let mut r: i64 = 0;
    for &x in a {
        if x > r + 1 {
            break;
        }
        r += x;
    }
    r + 1
}

fn main() {
    println!("{}", smallest_non_subset_sum(&[1, 2, 3, 10])); // 7
}
