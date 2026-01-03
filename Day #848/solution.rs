// Day 848: LRU cache with O(1) get/set using a hash map + intrusive doubly linked list
// implemented over a Vec arena (indices as pointers). Front = MRU, evict tail. O(1) per op.
use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

const NIL: usize = usize::MAX;

struct LRUCache {
    cap: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: usize, // MRU
    tail: usize, // LRU
    free: Vec<usize>,
}

impl LRUCache {
    fn new(cap: usize) -> Self {
        LRUCache { cap, map: HashMap::new(), nodes: Vec::new(), head: NIL, tail: NIL, free: Vec::new() }
    }
    fn unlink(&mut self, i: usize) {
        let (p, n) = (self.nodes[i].prev, self.nodes[i].next);
        if p != NIL { self.nodes[p].next = n; } else { self.head = n; }
        if n != NIL { self.nodes[n].prev = p; } else { self.tail = p; }
    }
    fn push_front(&mut self, i: usize) {
        self.nodes[i].prev = NIL;
        self.nodes[i].next = self.head;
        if self.head != NIL { self.nodes[self.head].prev = i; }
        self.head = i;
        if self.tail == NIL { self.tail = i; }
    }
    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&i) = self.map.get(&key) {
            self.unlink(i);
            self.push_front(i);
            Some(self.nodes[i].val)
        } else {
            None
        }
    }
    fn set(&mut self, key: i32, val: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = val;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        if self.map.len() == self.cap {
            let t = self.tail;
            self.unlink(t);
            self.map.remove(&self.nodes[t].key);
            self.free.push(t);
        }
        let idx = if let Some(f) = self.free.pop() {
            self.nodes[f] = Node { key, val, prev: NIL, next: NIL };
            f
        } else {
            self.nodes.push(Node { key, val, prev: NIL, next: NIL });
            self.nodes.len() - 1
        };
        self.push_front(idx);
        self.map.insert(key, idx);
    }
}

fn show(o: Option<i32>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let mut c = LRUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    println!("{}", show(c.get(1))); // 1
    c.set(3, 3);                    // evicts 2
    println!("{}", show(c.get(2))); // null
    c.set(4, 4);                    // evicts 1
    println!("{}", show(c.get(1))); // null
    println!("{}", show(c.get(3))); // 3
    println!("{}", show(c.get(4))); // 4
}
