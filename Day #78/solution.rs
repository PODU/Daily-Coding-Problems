// Day 78: Merge k sorted linked lists using a min-heap (BinaryHeap with Reverse).
// Time O(N log k) where N total nodes, Space O(k).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    // Flatten each list into a vector for index-based heap access.
    let mut cols: Vec<Vec<i32>> = Vec::new();
    for l in lists {
        let mut v = Vec::new();
        let mut cur = l.as_ref();
        while let Some(n) = cur {
            v.push(n.val);
            cur = n.next.as_ref();
        }
        cols.push(v);
    }
    let mut idx = vec![0usize; cols.len()];
    for (i, c) in cols.iter().enumerate() {
        if !c.is_empty() {
            heap.push(Reverse((c[0], i)));
        }
    }
    let mut head: Option<Box<ListNode>> = None;
    let mut tail: *mut Option<Box<ListNode>> = &mut head;
    while let Some(Reverse((val, i))) = heap.pop() {
        unsafe {
            *tail = Some(Box::new(ListNode { val, next: None }));
            tail = &mut (*tail).as_mut().unwrap().next;
        }
        idx[i] += 1;
        if idx[i] < cols[i].len() {
            heap.push(Reverse((cols[i][idx[i]], i)));
        }
    }
    head
}

fn build(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn main() {
    let lists = vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6])];
    let mut m = merge_k_lists(lists);
    let mut parts = Vec::new();
    while let Some(n) = m {
        parts.push(n.val.to_string());
        m = n.next;
    }
    println!("{}", parts.join(" -> ")); // 1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6
}
