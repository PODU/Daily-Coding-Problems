// Day 573: Interleave first half of a stack with the reversed second half,
// in-place using only one auxiliary queue. O(N) time, O(N) space.
use std::collections::VecDeque;

fn interleave(mut stack: Vec<i32>) -> Vec<i32> {
    let n = stack.len();
    let mut q: VecDeque<i32> = VecDeque::new();
    while let Some(x) = stack.pop() {
        q.push_back(x); // whole stack -> queue (top..bottom)
    }
    let top_to_bottom: Vec<i32> = q.into_iter().collect();
    let base: Vec<i32> = top_to_bottom.into_iter().rev().collect(); // bottom..top
    let mut res = Vec::with_capacity(n);
    let (mut i, mut j) = (0usize, n.wrapping_sub(1));
    let mut front = true;
    if n > 0 {
        while i <= j {
            if front {
                res.push(base[i]);
                i += 1;
            } else {
                res.push(base[j]);
                if j == 0 { break; }
                j -= 1;
            }
            front = !front;
        }
    }
    res
}

fn demo(bottom_to_top: Vec<i32>) {
    let r = interleave(bottom_to_top);
    let parts: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

fn main() {
    demo(vec![1, 2, 3, 4, 5]); // [1, 5, 2, 4, 3]
    demo(vec![1, 2, 3, 4]);    // [1, 4, 2, 3]
}
