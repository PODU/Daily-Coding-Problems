// Squares of a sorted array via two-pointer merge from both ends. O(n) time, O(n) space.

fn sorted_squares(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0i64; n];
    let mut l = 0usize;
    let mut r = n - 1;
    for i in (0..n).rev() {
        let ls = a[l] * a[l];
        let rs = a[r] * a[r];
        if ls > rs {
            res[i] = ls;
            l += 1;
        } else {
            res[i] = rs;
            if r > 0 { r -= 1; }
        }
    }
    res
}

fn main() {
    let input = [-9i64, -2, 0, 2, 3];
    let res = sorted_squares(&input);
    let parts: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
