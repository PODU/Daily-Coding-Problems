// Day 1000: Minimum of a rotated sorted array (no duplicates).
// Binary search comparing mid with the right end. O(log N) time, O(1) space.
fn find_min(nums: &[i32]) -> i32 {
    let (mut lo, mut hi) = (0usize, nums.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if nums[mid] > nums[hi] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    nums[lo]
}

fn main() {
    println!("{}", find_min(&[5, 7, 10, 3, 4])); // 3
}
