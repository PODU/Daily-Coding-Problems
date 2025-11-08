// Square a sorted array and return sorted. Two pointers from both ends pick larger
// absolute value into the back of the result. O(N) time, O(N) space.
fn sorted_squares(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0i64; n];
    let (mut l, mut r) = (0usize, n - 1);
    for k in (0..n).rev() {
        let lv = a[l] * a[l];
        let rv = a[r] * a[r];
        if lv > rv {
            res[k] = lv;
            l += 1;
        } else {
            res[k] = rv;
            if r > 0 { r -= 1; }
        }
    }
    res
}

fn main() {
    println!("{:?}", sorted_squares(&[-9, -2, 0, 2, 3])); // [0, 4, 4, 9, 81]
}
