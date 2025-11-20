// Day 636: Minimum in a rotated sorted array (no duplicates).
// Approach: binary search comparing mid with right endpoint.
// Time: O(log N), Space: O(1).
fn find_min(a: &[i32]) -> i32 {
    let (mut lo, mut hi) = (0usize, a.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] > a[hi] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    a[lo]
}

fn main() {
    println!("{}", find_min(&[5, 7, 10, 3, 4])); // 3
}
