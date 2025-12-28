// Non-decreasing with at most 1 modification: single pass counting violations,
// greedily lower nums[i-1] or raise nums[i]. Time O(n), Space O(1).

fn can_be_non_decreasing(input: &[i64]) -> bool {
    let mut nums = input.to_vec();
    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            count += 1;
            if count > 1 {
                return false;
            }
            if i < 2 || nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }
    true
}

fn main() {
    println!("{}", can_be_non_decreasing(&[10, 5, 7]));
    println!("{}", can_be_non_decreasing(&[10, 5, 1]));
}
