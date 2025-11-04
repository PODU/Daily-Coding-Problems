// Search in rotated sorted array via modified binary search. O(log n) time, O(1) space.
fn search(a: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0i64, a.len() as i64 - 1);
    while lo <= hi {
        let mid = (lo + (hi - lo) / 2) as usize;
        if a[mid] == target {
            return Some(mid);
        }
        if a[lo as usize] <= a[mid] {
            if a[lo as usize] <= target && target < a[mid] {
                hi = mid as i64 - 1;
            } else {
                lo = mid as i64 + 1;
            }
        } else if a[mid] < target && target <= a[hi as usize] {
            lo = mid as i64 + 1;
        } else {
            hi = mid as i64 - 1;
        }
    }
    None
}

fn main() {
    let arr = [13, 18, 25, 2, 8, 10];
    match search(&arr, 8) {
        Some(i) => println!("{}", i),
        None => println!("null"),
    }
    match search(&arr, 7) {
        Some(i) => println!("{}", i),
        None => println!("null"),
    }
}
