// Find the duplicate in array of length n+1 with values in {1..n}.
// Floyd's tortoise-and-hare cycle detection. Time O(n), Space O(1).

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
    let nums: Vec<usize> = vec![1, 2, 3, 4, 2]; // n = 4
    println!("{}", find_duplicate(&nums)); // expected: 2
}
