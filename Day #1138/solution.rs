// Modified binary search on a rotated sorted array. O(log n) time, O(1) space.
fn search(a: &[i32], target: i32) -> i32 {
    let (mut lo, mut hi) = (0i32, a.len() as i32 - 1);
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let m = a[mid as usize];
        if m == target {
            return mid;
        }
        if a[lo as usize] <= m {
            if a[lo as usize] <= target && target < m {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            if m < target && target <= a[hi as usize] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    -1
}

fn main() {
    let arr = [13, 18, 25, 2, 8, 10];
    println!("{}", search(&arr, 8));
}
