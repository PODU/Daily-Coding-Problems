// Find a peak in a rotated array (ends lowest) via binary search. O(log N) time, O(1) space.

fn find_peak(a: &[i32]) -> i32 {
    let (mut lo, mut hi) = (0usize, a.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < a[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    a[lo]
}

fn main() {
    let arr = [5, 7, 9, 3, 1];
    println!("{}", find_peak(&arr));
}
