// Day 763: Sliding window maximum via a monotonic decreasing deque of indices.
// Time: O(n), Space: O(k). Prints each window max as it is computed.
use std::collections::VecDeque;

fn sliding_max(a: &[i32], k: usize) {
    let mut dq: VecDeque<usize> = VecDeque::new(); // indices, values decreasing
    let mut out = Vec::new();
    for i in 0..a.len() {
        while let Some(&back) = dq.back() {
            if a[back] <= a[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        if *dq.front().unwrap() + k <= i {
            dq.pop_front();
        }
        if i + 1 >= k {
            out.push(a[*dq.front().unwrap()]);
        }
    }
    println!("{:?}", out);
}

fn main() {
    sliding_max(&[10, 5, 2, 7, 8, 7], 3); // [10, 7, 8, 8]
}
