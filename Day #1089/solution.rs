// Sliding window maximum via monotonic decreasing deque of indices. Time O(n), Space O(k).
use std::collections::VecDeque;

fn max_sliding_window(a: &[i32], k: usize) -> Vec<i32> {
    let mut dq: VecDeque<usize> = VecDeque::new(); // indices, values decreasing
    let mut res = Vec::new();
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
    let a = vec![10, 5, 2, 7, 8, 7];
    println!("{:?}", max_sliding_window(&a, 3));
}
