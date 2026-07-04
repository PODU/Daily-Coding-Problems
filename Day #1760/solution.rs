// Day 1760: Sliding window maximum.
// Approach: monotonic deque of indices (decreasing values). O(n) time, O(k) space.
use std::collections::VecDeque;

fn max_sliding_window(a: &[i32], k: usize) -> Vec<i32> {
    let mut res = Vec::new();
    let mut dq: VecDeque<usize> = VecDeque::new(); // indices, values decreasing
    for i in 0..a.len() {
        if let Some(&front) = dq.front() {
            if front + k <= i {
                dq.pop_front();
            }
        }
        while let Some(&back) = dq.back() {
            if a[back] <= a[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        if i + 1 >= k {
            res.push(a[*dq.front().unwrap()]);
        }
    }
    res
}

fn main() {
    let a = [10, 5, 2, 7, 8, 7];
    let k = 3;
    let res = max_sliding_window(&a, k);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
