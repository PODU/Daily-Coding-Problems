// Binary search for fixed point (A[i]==i) in sorted distinct array; A[i]-i non-decreasing.
// Time: O(log n), Space: O(1).
fn fixed_point(a: &[i64]) -> String {
    let (mut lo, mut hi, mut ans): (i64, i64, i64) = (0, a.len() as i64 - 1, -1);
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if a[mid as usize] == mid {
            ans = mid;
            hi = mid - 1;
        } else if a[mid as usize] < mid {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    if ans == -1 { "False".to_string() } else { ans.to_string() }
}

fn main() {
    println!("{}", fixed_point(&[-6, 0, 2, 40]));
    println!("{}", fixed_point(&[1, 5, 7, 8]));
}
