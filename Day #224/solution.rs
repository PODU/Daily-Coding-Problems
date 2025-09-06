// Day 224: Smallest positive integer not expressible as a subset sum (sorted array).
// Approach: greedy. Keep reachable range [1, ans-1]; if next a <= ans, extend by a, else ans is the gap.
// Time O(N), Space O(1).
fn smallest_non_subset_sum(a: &[i64]) -> i64 {
    let mut ans: i64 = 1; // smallest unreachable so far
    for &x in a {
        if x > ans {
            break;
        }
        ans += x;
    }
    ans
}

fn main() {
    println!("{}", smallest_non_subset_sum(&[1, 2, 3, 10])); // 7
}
