// Day 1666: GCD of n numbers.
// Approach: fold Euclid's algorithm across the list. Time O(n*log(max)), Space O(1).
fn gcd2(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn gcd_list(nums: &[i64]) -> i64 {
    nums.iter().fold(0, |g, &x| gcd2(g, x))
}

fn main() {
    println!("{}", gcd_list(&[42, 56, 14])); // 14
}
