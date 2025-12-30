// Fixed point in sorted distinct array via binary search (nums[i]-i non-decreasing).
// Time: O(log n), Space: O(1).

fn fixed_point(nums: &[i64]) -> Option<usize> {
    let mut lo: i64 = 0;
    let mut hi: i64 = nums.len() as i64 - 1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if nums[mid as usize] == mid {
            return Some(mid as usize);
        } else if nums[mid as usize] < mid {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

fn main() {
    match fixed_point(&[-6, 0, 2, 40]) {
        Some(i) => println!("{}", i),
        None => println!("False"),
    }
    match fixed_point(&[1, 5, 7, 8]) {
        Some(i) => println!("{}", i),
        None => println!("False"),
    }
}
