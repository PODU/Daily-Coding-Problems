// Day 1063: Find a peak in a rotated sorted array of distinct values.
// Approach: binary search; if a[mid] < a[mid+1] go right else left. Time O(log n), Space O(1).

fn find_peak(a: &[i32]) -> usize {
    let mut lo = 0usize;
    let mut hi = a.len() - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < a[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo // index of the peak
}

fn main() {
    let a = [3, 4, 5, 1, 2];
    let idx = find_peak(&a);
    println!("{}", a[idx]); // 5
}
