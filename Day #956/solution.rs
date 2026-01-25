// Day 956: merge k sorted singly linked lists using a min-heap.
// Time O(N log k), Space O(k) where N = total nodes.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Box-based singly linked list.
#[derive(Default)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn build(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn merge_k(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // Heap stores (value, list-index); we keep current heads in a vec.
    let mut heads = lists;
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for (i, h) in heads.iter().enumerate() {
        if let Some(n) = h {
            heap.push(Reverse((n.val, i)));
        }
    }
    let mut dummy = Box::new(ListNode::default());
    let mut tail = &mut dummy;
    while let Some(Reverse((_, i))) = heap.pop() {
        let mut node = heads[i].take().unwrap();
        heads[i] = node.next.take();
        if let Some(n) = &heads[i] {
            heap.push(Reverse((n.val, i)));
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {
    let lists = vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6])];
    let merged = merge_k(lists);
    let mut out = Vec::new();
    let mut cur = merged.as_ref();
    while let Some(n) = cur {
        out.push(n.val.to_string());
        cur = n.next.as_ref();
    }
    println!("{}", out.join(" ")); // 1 1 2 3 4 4 5 6
}
