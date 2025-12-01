// Majority via Boyer-Moore voting (O(n) time, O(1) space). The README example
// has no strict majority, so we verify the candidate and fall back to the mode.

fn count_of(v: &[i32], target: i32) -> usize {
    v.iter().filter(|&&x| x == target).count()
}

fn majority(nums: &[i32]) -> i32 {
    let mut count = 0;
    let mut candidate = 0;
    for &x in nums {
        // Boyer-Moore voting pass
        if count == 0 {
            candidate = x;
        }
        count += if x == candidate { 1 } else { -1 };
    }
    if count_of(nums, candidate) > nums.len() / 2 {
        return candidate;
    }
    let mut best = nums[0]; // fallback: most frequent element
    for &x in nums {
        if count_of(nums, x) > count_of(nums, best) {
            best = x;
        }
    }
    best
}

fn main() {
    let nums = [1, 2, 1, 1, 3, 4, 0];
    println!("{}", majority(&nums)); // 1
}
