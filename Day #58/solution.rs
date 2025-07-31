// Day 58: Search in rotated sorted array via modified binary search.
// Time: O(log n), Space: O(1). Returns None if absent.
fn search(a: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0i32, a.len() as i32 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let m = mid as usize;
        if a[m] == target {
            return Some(m);
        }
        if a[lo as usize] <= a[m] {
            // left half sorted
            if a[lo as usize] <= target && target < a[m] {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            // right half sorted
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
    match search(&[13, 18, 25, 2, 8, 10], 8) {
        Some(i) => println!("{}", i), // 4
        None => println!("null"),
    }
}
