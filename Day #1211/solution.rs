// Day 1211: Interleave first half of a stack with the reversed second half, using one queue.
// Load stack bottom->top into the queue, then alternately push front/back (back via rotation). Time O(n^2), Space O(n).
use std::collections::VecDeque;

fn interleave(st: &mut Vec<i32>) {
    let mut q: VecDeque<i32> = VecDeque::new();
    let mut n = 0;
    while let Some(x) = st.pop() {
        q.push_back(x); // front = old top
        n += 1;
    }
    for _ in 0..n {
        st.push(q.pop_front().unwrap()); // top = old bottom
    }
    for _ in 0..n {
        q.push_back(st.pop().unwrap()); // front = bottom
    }
    let mut remaining = n;
    let mut take_front = true;
    while remaining > 0 {
        if take_front {
            st.push(q.pop_front().unwrap());
        } else {
            for _ in 0..remaining - 1 {
                let f = q.pop_front().unwrap();
                q.push_back(f);
            }
            st.push(q.pop_front().unwrap());
        }
        remaining -= 1;
        take_front = !take_front;
    }
}

fn main() {
    let mut st = vec![1, 2, 3, 4, 5]; // bottom -> top
    interleave(&mut st);
    println!("{:?}", st); // [1, 5, 2, 4, 3]
}
