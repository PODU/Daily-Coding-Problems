// Approach: Monotonic deque of indices; front always holds the window max. O(n) time, O(k) space.
use std::collections::VecDeque;

fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut dq: VecDeque<usize> = VecDeque::new(); // indices, decreasing values
    let mut res = Vec::new();
    for i in 0..nums.len() {
        if let Some(&front) = dq.front() {
            if front + k <= i {
                dq.pop_front();
            }
        }
        while let Some(&back) = dq.back() {
            if nums[back] <= nums[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        if i + 1 >= k {
            res.push(nums[*dq.front().unwrap()]);
        }
    }
    res
}

fn main() {
    let nums = [10, 5, 2, 7, 8, 7];
    let k = 3;
    let res = max_sliding_window(&nums, k);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
