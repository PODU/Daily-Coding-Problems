// Day 1429: Find a peak element (greater than both neighbors) in O(log N).
// Approach: binary search; if a[mid] < a[mid+1] a peak lies right, else left/at mid.
// Time: O(log n), Space: O(1).

fn find_peak(a: &[i32]) -> usize {
    let (mut lo, mut hi) = (0usize, a.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < a[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    let a = [1, 3, 5, 7, 6, 4, 2];
    println!("{}", a[find_peak(&a)]); // 7
}
