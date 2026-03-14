// Day 1204: GCD of n numbers.
// Fold Euclidean gcd across the list. Time O(n log max), Space O(1).
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn gcd_all(nums: &[u64]) -> u64 {
    nums.iter().fold(0, |g, &x| gcd(g, x))
}

fn main() {
    println!("{}", gcd_all(&[42, 56, 14])); // 14
}
