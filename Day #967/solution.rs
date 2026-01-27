// Day 967: Find the duplicate in array of n+1 ints from {1..n}.
// Approach: Floyd's tortoise & hare on value->index cycle. Time O(n), Space O(1).

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
    println!("{}", find_duplicate(&[1, 3, 4, 2, 2])); // 2
}
