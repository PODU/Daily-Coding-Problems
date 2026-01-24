// Day 951: interleave first half of a stack with the reversed second half,
// in place using only one auxiliary queue (push/pop, enqueue/dequeue).
// Time O(n^2) due to rotations, Space O(1) extra besides the queue.
use std::collections::VecDeque;

fn interleave(input: &[i64]) -> Vec<i64> {
    let mut st: Vec<i64> = input.to_vec(); // top = end
    let mut q: VecDeque<i64> = VecDeque::new();
    while let Some(x) = st.pop() {
        q.push_back(x);
    }
    while let Some(x) = q.pop_front() {
        st.push(x);
    }
    while let Some(x) = st.pop() {
        q.push_back(x);
    }
    // q = a0..a_{n-1}
    while let Some(front) = q.pop_front() {
        st.push(front);
        let m = q.len();
        if m == 0 {
            break;
        }
        for _ in 0..m - 1 {
            let x = q.pop_front().unwrap();
            q.push_back(x);
        }
        st.push(q.pop_front().unwrap());
    }
    st
}

fn main() {
    println!("{:?}", interleave(&[1, 2, 3, 4, 5])); // [1, 5, 2, 4, 3]
    println!("{:?}", interleave(&[1, 2, 3, 4]));    // [1, 4, 2, 3]
}
