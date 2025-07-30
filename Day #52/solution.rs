// Day 52: LRU cache with hashmap + intrusive doubly linked list (vector arena).
// Time: O(1) per op, Space: O(n).
use std::collections::HashMap;

struct LNode {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

const NIL: usize = usize::MAX;

struct LRUCache {
    cap: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<LNode>,
    free: Vec<usize>,
    head: usize, // MRU
    tail: usize, // LRU
}

impl LRUCache {
    fn new(n: usize) -> Self {
        LRUCache { cap: n, map: HashMap::new(), nodes: Vec::new(), free: Vec::new(), head: NIL, tail: NIL }
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
            Some(self.nodes[i].value)
        } else {
            None
        }
    }

    fn set(&mut self, key: i32, value: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].value = value;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        if self.map.len() == self.cap {
            let lru = self.tail;
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.free.push(lru);
        }
        let node = LNode { key, value, prev: NIL, next: NIL };
        let i = if let Some(f) = self.free.pop() {
            self.nodes[f] = node;
            f
        } else {
            self.nodes.push(node);
            self.nodes.len() - 1
        };
        self.push_front(i);
        self.map.insert(key, i);
    }
}

fn show(v: Option<i32>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let mut c = LRUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    println!("{}", show(c.get(1))); // 1
    c.set(3, 3);                    // evicts key 2
    println!("{}", show(c.get(2))); // null
    c.set(4, 4);                    // evicts key 1
    println!("{}", show(c.get(1))); // null
    println!("{}", show(c.get(3))); // 3
    println!("{}", show(c.get(4))); // 4
}
