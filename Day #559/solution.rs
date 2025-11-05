// Merge k sorted linked lists using a BinaryHeap (min-heap via Reverse) of heads.
// Time: O(N log k), Space: O(k) for the heap.
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // Heap stores (value, owned remaining-from-this-node) keyed by value (min via Reverse).
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    let mut nodes: Vec<Option<Box<ListNode>>> = lists;
    for (i, n) in nodes.iter().enumerate() {
        if let Some(node) = n {
            heap.push(Reverse((node.val, i)));
        }
    }
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;
    while let Some(Reverse((_, i))) = heap.pop() {
        // Detach head node from slot i.
        let mut node = nodes[i].take().unwrap();
        let next = node.next.take();
        if let Some(nx) = next {
            let v = nx.val;
            nodes[i] = Some(nx);
            heap.push(Reverse((v, i)));
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {
    let lists = vec![
        build_list(&[1, 4, 5]),
        build_list(&[1, 3, 4]),
        build_list(&[2, 6]),
    ];
    let merged = merge_k_lists(lists);
    let mut parts: Vec<String> = Vec::new();
    let mut cur = &merged;
    while let Some(node) = cur {
        parts.push(node.val.to_string());
        cur = &node.next;
    }
    println!("{}", parts.join(" "));
}
