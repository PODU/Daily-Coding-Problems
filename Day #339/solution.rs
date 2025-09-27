// Three entries summing to k: sort + fix one + two-pointer.
// O(n^2) time, O(1) extra space.

fn three_sum(mut a: Vec<i64>, k: i64) -> bool {
    a.sort();
    let n = a.len();
    if n < 3 {
        return false;
    }
    for i in 0..n - 2 {
        let (mut lo, mut hi) = (i + 1, n - 1);
        let target = k - a[i];
        while lo < hi {
            let s = a[lo] + a[hi];
            if s == target {
                return true;
            }
            if s < target {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    false
}

fn main() {
    let a = vec![20, 303, 3, 4, 25];
    println!("{}", three_sum(a, 49)); // true
}
