// Day 662: GCD of n numbers via repeated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
fn gcd2(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn gcd_all(v: &[u64]) -> u64 {
    v.iter().fold(0u64, |g, &x| gcd2(g, x))
}

fn main() {
    println!("{}", gcd_all(&[42, 56, 14])); // 14
}
