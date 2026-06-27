// Day 1723: Search element in rotated sorted array.
// Modified binary search: one half is always sorted; decide which side to recurse.
// Time: O(log n), Space: O(1). Returns Some(index) or None if absent.

fn search(a: &[i32], target: i32) -> Option<usize> {
    if a.is_empty() {
        return None;
    }
    let (mut lo, mut hi) = (0isize, a.len() as isize - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let (m, l, h) = (a[mid as usize], a[lo as usize], a[hi as usize]);
        if m == target {
            return Some(mid as usize);
        }
        if l <= m {
            // left half sorted
            if l <= target && target < m {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            // right half sorted
            if m < target && target <= h {
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
    match search(&a, 8) {
        Some(i) => println!("{}", i), // 4
        None => println!("null"),
    }
    match search(&a, 99) {
        Some(i) => println!("{}", i),
        None => println!("null"), // not found
    }
}
