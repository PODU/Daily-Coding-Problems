// LRU cache via HashMap + intrusive doubly linked list stored in a Vec arena (index links). O(1) per get/set. Space O(capacity).
use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    cap: usize,
    nodes: Vec<Node>,
    map: HashMap<i32, usize>,
    head: usize, // sentinel MRU side
    tail: usize, // sentinel LRU side
    free: Vec<usize>,
}

impl LRUCache {
    fn new(n: usize) -> Self {
        // index 0 = head sentinel, index 1 = tail sentinel
        let mut nodes = Vec::with_capacity(n + 2);
        nodes.push(Node { key: 0, val: 0, prev: 0, next: 1 }); // head
        nodes.push(Node { key: 0, val: 0, prev: 0, next: 1 }); // tail
        LRUCache { cap: n, nodes, map: HashMap::new(), head: 0, tail: 1, free: Vec::new() }
    }

    fn detach(&mut self, idx: usize) {
        let (p, n) = (self.nodes[idx].prev, self.nodes[idx].next);
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn attach_front(&mut self, idx: usize) {
        let first = self.nodes[self.head].next;
        self.nodes[idx].prev = self.head;
        self.nodes[idx].next = first;
        self.nodes[first].prev = idx;
        self.nodes[self.head].next = idx;
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&idx) = self.map.get(&key) {
            self.detach(idx);
            self.attach_front(idx);
            Some(self.nodes[idx].val)
        } else {
            None
        }
    }

    fn set(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].val = value;
            self.detach(idx);
            self.attach_front(idx);
            return;
        }
        if self.map.len() == self.cap {
            let lru = self.nodes[self.tail].prev;
            self.detach(lru);
            let old_key = self.nodes[lru].key;
            self.map.remove(&old_key);
            self.free.push(lru);
        }
        let idx = if let Some(i) = self.free.pop() {
            self.nodes[i].key = key;
            self.nodes[i].val = value;
            i
        } else {
            self.nodes.push(Node { key, val: value, prev: 0, next: 0 });
            self.nodes.len() - 1
        };
        self.attach_front(idx);
        self.map.insert(key, idx);
    }
}

fn print_get(c: &mut LRUCache, key: i32) {
    match c.get(key) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
}

fn main() {
    let mut c = LRUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    print_get(&mut c, 1); // 1
    c.set(3, 3);          // evicts 2
    print_get(&mut c, 2); // null
    c.set(4, 4);          // evicts 1
    print_get(&mut c, 1); // null
    print_get(&mut c, 3); // 3
    print_get(&mut c, 4); // 4
}
