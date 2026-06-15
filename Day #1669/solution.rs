// Two-pointer merge from both ends of sorted array; larger abs square goes to back. O(n) time, O(n) space.
fn sorted_squares(a: &[i64]) -> Vec<i64> {
    let n = a.len(); let mut r = vec![0i64; n]; let (mut l, mut h) = (0usize, n - 1);
    for p in (0..n).rev() {
        let lo = a[l] * a[l]; let hi = a[h] * a[h];
        if lo > hi { r[p] = lo; l += 1; } else { r[p] = hi; if h > 0 { h -= 1; } }
    }
    r
}
fn main() {
    let r = sorted_squares(&[-9, -2, 0, 2, 3]);
    let s: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("[{}]", s.join(", "));
}
