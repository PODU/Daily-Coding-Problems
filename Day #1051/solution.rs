// Search in rotated sorted array via modified binary search.
// Time: O(log n), Space: O(1).
fn search_rotated(a: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0i64, a.len() as i64 - 1);
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let m = mid as usize;
        if a[m] == target {
            return Some(m);
        }
        if a[lo as usize] <= a[m] {
            if a[lo as usize] <= target && target < a[m] {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            if a[m] < target && target <= a[hi as usize] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    None
}

fn main() {
    let a = [13, 18, 25, 2, 8, 10];
    match search_rotated(&a, 8) {
        Some(i) => println!("{}", i), // 4
        None => println!("null"),
    }
}
