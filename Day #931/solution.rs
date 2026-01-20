// Day 931: GCD of n numbers by folding pairwise gcd.
// Time: O(n * log(maxVal)), Space: O(1).
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn gcd_list(nums: &[u64]) -> u64 {
    nums.iter().fold(0u64, |g, &x| gcd(g, x))
}

fn main() {
    println!("{}", gcd_list(&[42, 56, 14])); // 14
}
