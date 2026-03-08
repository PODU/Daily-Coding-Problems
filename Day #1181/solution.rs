// Day 1181: Find the minimum in a rotated sorted array (no duplicates).
// Binary search: compare mid to the right end to discard the sorted half.
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
    println!("{}", find_min(&[5, 7, 10, 3, 4])); // 3
}
