// Day 1761: Interleave first half of a stack with the reversed second half,
// in-place using ONE auxiliary queue (only push/pop/enqueue/dequeue).
// Dump stack into queue (reverses), rebuild taking alternately back/front.
// Time O(n^2), Space O(n). Stack modeled as Vec (end = top).
use std::collections::VecDeque;

fn interleave(input: &[i32]) -> Vec<i32> {
    let mut stack: Vec<i32> = input.to_vec(); // end = top
    let mut q: VecDeque<i32> = VecDeque::new();
    while let Some(top) = stack.pop() {
        q.push_back(top);
    }
    let mut take_back = true;
    while !q.is_empty() {
        let v = if take_back {
            for _ in 0..q.len() - 1 {
                let f = q.pop_front().unwrap();
                q.push_back(f);
            }
            q.pop_front().unwrap()
        } else {
            q.pop_front().unwrap()
        };
        stack.push(v);
        take_back = !take_back;
    }
    stack
}

fn fmt(v: &[i32]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    println!("{}", fmt(&interleave(&[1, 2, 3, 4, 5])));
    println!("{}", fmt(&interleave(&[1, 2, 3, 4])));
}
