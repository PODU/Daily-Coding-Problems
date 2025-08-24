// Day 164: Find duplicate via Floyd's cycle detection (values as next-pointers).
// O(n) time, O(1) extra space (beats the O(n)-space requirement).

fn find_duplicate(nums: &[usize]) -> usize {
    let mut slow = nums[0];
    let mut fast = nums[0];
    loop {
        slow = nums[slow];
        fast = nums[nums[fast]];
        if slow == fast {
            break;
        }
    }
    slow = nums[0];
    while slow != fast {
        slow = nums[slow];
        fast = nums[fast];
    }
    slow
}

fn main() {
    let nums = [1usize, 2, 3, 4, 2]; // n = 4, length n+1
    println!("{}", find_duplicate(&nums)); // 2
}
