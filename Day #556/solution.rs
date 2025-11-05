// Non-decreasing with at most one modification: single pass, greedy fix. O(n) time, O(1) space.
fn check_possibility(input: &[i32]) -> bool {
    let mut nums = input.to_vec();
    let mut cnt = 0;
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            cnt += 1;
            if cnt > 1 {
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
    println!("{}", check_possibility(&[10, 5, 7]));
    println!("{}", check_possibility(&[10, 5, 1]));
}
