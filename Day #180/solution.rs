// Interleave first half with reversed second half using ONE auxiliary queue.
// Result (bottom->top) = arr[0],arr[n-1],arr[1],arr[n-2],...  O(n^2) time, O(n) space.
use std::collections::VecDeque;

fn interleave(arr: &[i32]) -> Vec<i32> {
    let mut stack: Vec<i32> = arr.to_vec(); // top = end
    let mut q: VecDeque<i32> = VecDeque::new();

    while let Some(v) = stack.pop() { q.push_back(v); }       // A: stack -> queue
    while let Some(v) = q.pop_front() { stack.push(v); }      // B: queue -> stack (reverse)
    while let Some(v) = stack.pop() { q.push_back(v); }       // C: stack -> queue (front..back = bottom..top)

    let mut take_front = true;
    while !q.is_empty() {
        if take_front {
            stack.push(q.pop_front().unwrap());
        } else {
            for _ in 0..q.len() - 1 { // rotate back to front
                let x = q.pop_front().unwrap();
                q.push_back(x);
            }
            stack.push(q.pop_front().unwrap());
        }
        take_front = !take_front;
    }
    stack
}

fn fmt(a: &[i32]) -> String {
    let s: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    format!("[{}]", s.join(", "))
}

fn main() {
    println!("{}", fmt(&interleave(&[1, 2, 3, 4, 5])));
    println!("{}", fmt(&interleave(&[1, 2, 3, 4])));
}
