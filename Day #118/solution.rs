// Day 118: Two-pointer merge from both ends into result back-to-front. O(n) time, O(n) space.
fn sorted_squares(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0i64; n];
    let (mut lo, mut hi) = (0usize, n - 1);
    for k in (0..n).rev() {
        let sl = a[lo] * a[lo];
        let sh = a[hi] * a[hi];
        if sl > sh {
            res[k] = sl;
            lo += 1;
        } else {
            res[k] = sh;
            if hi > 0 { hi -= 1; }
        }
    }
    res
}

fn main() {
    println!("{:?}", sorted_squares(&[-9, -2, 0, 2, 3])); // [0, 4, 4, 9, 81]
}
