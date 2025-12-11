// Day 735: Find any peak element in O(log N).
// Approach: Binary search - move toward the larger neighbor; a peak must lie that way.
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
    let a = [0, 2, 5, 3, 1];
    let i = find_peak(&a);
    println!("Peak element: {} (index {})", a[i], i); // 5 (index 2)
}
