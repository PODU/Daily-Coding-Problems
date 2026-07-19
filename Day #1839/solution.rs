// Day 1839: Minimum of a rotated sorted array (no duplicates) via binary search.
// Time O(log N), Space O(1).

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
    println!("{}", find_min(&[5, 7, 10, 3, 4]));
}
