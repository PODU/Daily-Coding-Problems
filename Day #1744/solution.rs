// Merge k sorted linked lists via min-heap (BinaryHeap+Reverse) of current heads. O(N log k) time, O(k) space.
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn build(vals: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn merge_k(lists: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
    // Heap holds (value, sequence id) to pull smallest; nodes stored separately.
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    let mut cursors: Vec<Option<Box<Node>>> = lists;
    for (i, l) in cursors.iter().enumerate() {
        if let Some(n) = l {
            heap.push(Reverse((n.val, i)));
        }
    }
    let mut dummy = Box::new(Node { val: 0, next: None });
    let mut tail = &mut dummy;
    while let Some(Reverse((_, i))) = heap.pop() {
        let mut node = cursors[i].take().unwrap();
        let next = node.next.take();
        cursors[i] = next;
        if let Some(n) = &cursors[i] {
            heap.push(Reverse((n.val, i)));
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {
    let lists = vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6])];
    let m = merge_k(lists);
    let mut out: Vec<String> = Vec::new();
    let mut cur = &m;
    while let Some(n) = cur {
        out.push(n.val.to_string());
        cur = &n.next;
    }
    println!("{}", out.join(" "));
}
