// Huffman coding: BinaryHeap (as min-heap via reversed Ord) merges two smallest nodes, then DFS assigns codes.
// Left='0', right='1'; tie-break by insertion order for determinism. O(k log k) time, O(k) space.
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Node {
    freq: i32,
    order: i32,
    ch: Option<char>,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, o: &Self) -> bool { self.freq == o.freq && self.order == o.order }
}
impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, o: &Self) -> Ordering {
        // reverse so BinaryHeap (max-heap) pops smallest freq, then smallest order
        o.freq.cmp(&self.freq).then(o.order.cmp(&self.order))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) }
}

fn dfs(n: &Node, code: String, codes: &mut Vec<(char, String)>) {
    if n.l.is_none() && n.r.is_none() {
        let c = if code.is_empty() { "0".to_string() } else { code };
        codes.push((n.ch.unwrap(), c));
        return;
    }
    dfs(n.l.as_ref().unwrap(), format!("{}0", code), codes);
    dfs(n.r.as_ref().unwrap(), format!("{}1", code), codes);
}

fn huffman(freqs: &[(char, i32)]) -> Vec<(char, String)> {
    let mut sorted = freqs.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut heap = BinaryHeap::new();
    let mut order = 0;
    for (ch, f) in sorted {
        heap.push(Node { freq: f, order, ch: Some(ch), l: None, r: None });
        order += 1;
    }
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        heap.push(Node {
            freq: a.freq + b.freq,
            order,
            ch: None,
            l: Some(Box::new(a)),
            r: Some(Box::new(b)),
        });
        order += 1;
    }
    let root = heap.pop().unwrap();
    let mut codes = Vec::new();
    dfs(&root, String::new(), &mut codes);
    codes.sort_by(|a, b| a.0.cmp(&b.0));
    codes
}

fn main() {
    let freqs = vec![('a', 5), ('b', 9), ('c', 12), ('d', 13), ('e', 16), ('f', 45)];
    for (ch, code) in huffman(&freqs) {
        println!("{}: {}", ch, code);
    }
}
