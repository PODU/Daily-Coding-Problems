// 3-sum decision: sort, fix one element, two-pointer scan the rest. O(n^2) time, O(1) extra.
fn three_sum(mut a: Vec<i64>, k: i64) -> bool {
    a.sort();
    let n = a.len();
    for i in 0..n.saturating_sub(2) {
        let (mut lo, mut hi) = (i + 1, n - 1);
        while lo < hi {
            let s = a[i] + a[lo] + a[hi];
            if s == k {
                return true;
            }
            if s < k {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    false
}

fn main() {
    println!("{}", three_sum(vec![20, 303, 3, 4, 25], 49));
}
