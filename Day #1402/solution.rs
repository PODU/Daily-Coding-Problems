// Huffman coding: merge the two lowest-frequency nodes via a min-heap to build
// an optimal prefix tree, then DFS to assign codes (left=0, right=1).
// Time O(C log C) for C distinct chars, Space O(C).
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node {
    freq: i32,
    order: i32,
    ch: Option<u8>,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

// Wrapper so BinaryHeap (a max-heap) behaves as a min-heap on (freq, order).
struct HeapItem(Node);
impl PartialEq for HeapItem {
    fn eq(&self, o: &Self) -> bool {
        self.0.freq == o.0.freq && self.0.order == o.0.order
    }
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

fn dfs(n: &Node, path: String, codes: &mut Vec<(u8, String)>) {
    if n.l.is_none() && n.r.is_none() {
        let p = if path.is_empty() { "0".to_string() } else { path };
        codes.push((n.ch.unwrap(), p));
        return;
    }
    dfs(n.l.as_ref().unwrap(), format!("{}0", path), codes);
    dfs(n.r.as_ref().unwrap(), format!("{}1", path), codes);
}

fn main() {
    let freq: Vec<(u8, i32)> = vec![(b'c', 1), (b'a', 4), (b't', 3), (b's', 2)];
    let mut heap = BinaryHeap::new();
    let mut cnt = 0;
    for (ch, f) in &freq {
        heap.push(HeapItem(Node { freq: *f, order: cnt, ch: Some(*ch), l: None, r: None }));
        cnt += 1;
    }
    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let merged = Node {
            freq: a.freq + b.freq,
            order: cnt,
            ch: None,
            l: Some(Box::new(a)),
            r: Some(Box::new(b)),
        };
        cnt += 1;
        heap.push(HeapItem(merged));
    }
    let root = heap.pop().unwrap().0;
    let mut codes = Vec::new();
    dfs(&root, String::new(), &mut codes);
    codes.sort_by_key(|x| x.0);
    let map: std::collections::HashMap<u8, String> = codes.iter().cloned().collect();
    for (ch, code) in &codes {
        println!("{}: {}", *ch as char, code);
    }
    let enc: String = "cats".bytes().map(|c| map[&c].clone()).collect();
    println!("encode(cats): {}", enc);
}
