// Day 528: Huffman coding. Build a prefix tree by repeatedly merging the two
// lowest-frequency nodes (min-heap), then read each char's code from its
// root-to-leaf path (left=0, right=1). Time O(n log n), space O(n).
// Note: the README's example tree is illustrative, not a strict Huffman tree;
// this produces a correct, deterministic optimal-prefix Huffman mapping.
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap};

struct Node {
    freq: i64,
    order: i32,
    ch: Option<char>,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

// Wrap for a min-heap (BinaryHeap is a max-heap): reverse ordering on (freq, order).
struct HeapItem(Node);
impl PartialEq for HeapItem {
    fn eq(&self, o: &Self) -> bool {
        self.0.freq == o.0.freq && self.0.order == o.0.order
    }
}
impl Eq for HeapItem {}
impl Ord for HeapItem {
    fn cmp(&self, o: &Self) -> Ordering {
        // reversed so the smallest (freq, order) is at the top
        o.0.freq.cmp(&self.0.freq).then(o.0.order.cmp(&self.0.order))
    }
}
impl PartialOrd for HeapItem {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) }
}

fn build_codes(n: &Node, path: String, codes: &mut BTreeMap<char, String>) {
    if n.l.is_none() && n.r.is_none() {
        codes.insert(n.ch.unwrap(), if path.is_empty() { "0".into() } else { path });
        return;
    }
    build_codes(n.l.as_ref().unwrap(), format!("{}0", path), codes);
    build_codes(n.r.as_ref().unwrap(), format!("{}1", path), codes);
}

fn main() {
    let freq = vec![('c', 1i64), ('a', 1), ('t', 1), ('s', 1)];

    let mut heap = BinaryHeap::new();
    let mut order = 0;
    for (ch, f) in freq {
        heap.push(HeapItem(Node { freq: f, order, ch: Some(ch), l: None, r: None }));
        order += 1;
    }
    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        heap.push(HeapItem(Node {
            freq: a.freq + b.freq,
            order,
            ch: None,
            l: Some(Box::new(a)),
            r: Some(Box::new(b)),
        }));
        order += 1;
    }
    let root = heap.pop().unwrap().0;

    let mut codes = BTreeMap::new();
    build_codes(&root, String::new(), &mut codes);
    for (ch, code) in &codes {
        println!("{} -> {}", ch, code);
    }

    let word = "cats";
    let encoded: String = word.chars().map(|c| codes[&c].clone()).collect();
    println!("{} -> {}", word, encoded);
}
