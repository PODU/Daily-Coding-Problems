// Day 424: Two unique elements via XOR partition. O(n) time, O(1) space.
// XOR all -> a^b; isolate a low set bit; partition & XOR each group -> a, b.
fn main() {
    let a = [2i64, 4, 6, 8, 10, 2, 6, 10];
    let mut x = 0i64;
    for &v in &a {
        x ^= v;
    }
    let bit = x & (-x);
    let (mut p, mut q) = (0i64, 0i64);
    for &v in &a {
        if v & bit != 0 {
            p ^= v;
        } else {
            q ^= v;
        }
    }
    let (lo, hi) = (p.min(q), p.max(q));
    println!("{} and {}", lo, hi);
}
