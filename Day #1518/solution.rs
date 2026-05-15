// Huffman coding: build tree via min-heap, merge two smallest nodes, assign 0/1 edges.
// Time: O(n log n) for n symbols. Space: O(n).
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node {
    ch: Option<char>,
    freq: i64,
    order: usize,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

// Wrapper to make BinaryHeap behave as a min-heap by (freq, order).
struct HeapItem(Box<Node>);
impl PartialEq for HeapItem {
    fn eq(&self, o: &Self) -> bool { self.0.freq == o.0.freq && self.0.order == o.0.order }
}
impl Eq for HeapItem {}
impl Ord for HeapItem {
    fn cmp(&self, o: &Self) -> Ordering {
        // reverse for min-heap
        o.0.freq.cmp(&self.0.freq).then(o.0.order.cmp(&self.0.order))
    }
}
impl PartialOrd for HeapItem {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) }
}

fn build(n: &Node, code: String, out: &mut Vec<(char, String)>) {
    if n.l.is_none() && n.r.is_none() {
        let c = if code.is_empty() { "0".to_string() } else { code };
        out.push((n.ch.unwrap(), c));
        return;
    }
    if let Some(ref l) = n.l { build(l, format!("{}0", code), out); }
    if let Some(ref r) = n.r { build(r, format!("{}1", code), out); }
}

fn main() {
    let freqs: Vec<(char, i64)> = vec![('a',5),('b',9),('c',12),('d',13),('e',16),('f',45)];
    let mut heap = BinaryHeap::new();
    let mut counter = 0usize;
    for &(ch, f) in &freqs {
        heap.push(HeapItem(Box::new(Node { ch: Some(ch), freq: f, order: counter, l: None, r: None })));
        counter += 1;
    }
    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let m = Node { ch: None, freq: a.freq + b.freq, order: counter, l: Some(a), r: Some(b) };
        counter += 1;
        heap.push(HeapItem(Box::new(m)));
    }
    let root = heap.pop().unwrap().0;
    let mut codes: Vec<(char, String)> = Vec::new();
    build(&root, String::new(), &mut codes);
    codes.sort_by_key(|&(c, _)| c);
    let fmap: std::collections::HashMap<char, i64> = freqs.iter().cloned().collect();
    let mut total_bits: i64 = 0;
    for (ch, code) in &codes {
        println!("{}: {}", ch, code);
        total_bits += code.len() as i64 * fmap[ch];
    }
    println!("Total encoded bits: {}", total_bits);
}
