// LRU cache via HashMap + index-based doubly linked list (Vec arena); get/set O(1). O(1) time, O(n) space.
use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    cap: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: usize, // sentinel index 0 (most recent side)
    tail: usize, // sentinel index 1 (least recent side)
    free: Vec<usize>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        let mut nodes = Vec::new();
        // index 0 = head sentinel, index 1 = tail sentinel
        nodes.push(Node { key: 0, val: 0, prev: 0, next: 1 });
        nodes.push(Node { key: 0, val: 0, prev: 0, next: 1 });
        LRUCache {
            cap: capacity,
            map: HashMap::new(),
            nodes,
            head: 0,
            tail: 1,
            free: Vec::new(),
        }
    }

    fn unlink(&mut self, i: usize) {
        let p = self.nodes[i].prev;
        let n = self.nodes[i].next;
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn add_front(&mut self, i: usize) {
        let first = self.nodes[self.head].next;
        self.nodes[i].prev = self.head;
        self.nodes[i].next = first;
        self.nodes[first].prev = i;
        self.nodes[self.head].next = i;
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&i) = self.map.get(&key) {
            self.unlink(i);
            self.add_front(i);
            Some(self.nodes[i].val)
        } else {
            None
        }
    }

    fn set(&mut self, key: i32, value: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = value;
            self.unlink(i);
            self.add_front(i);
            return;
        }
        if self.map.len() == self.cap {
            let lru = self.nodes[self.tail].prev;
            self.unlink(lru);
            let k = self.nodes[lru].key;
            self.map.remove(&k);
            self.free.push(lru);
        }
        let idx = if let Some(idx) = self.free.pop() {
            self.nodes[idx] = Node { key, val: value, prev: 0, next: 0 };
            idx
        } else {
            self.nodes.push(Node { key, val: value, prev: 0, next: 0 });
            self.nodes.len() - 1
        };
        self.add_front(idx);
        self.map.insert(key, idx);
    }
}

fn fmt(v: Option<i32>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.set(1, 1);
    cache.set(2, 2);
    println!("{}", fmt(cache.get(1))); // 1
    cache.set(3, 3);                   // evicts key 2
    println!("{}", fmt(cache.get(2))); // null
    cache.set(4, 4);                   // evicts key 1
    println!("{}", fmt(cache.get(1))); // null
    println!("{}", fmt(cache.get(3))); // 3
    println!("{}", fmt(cache.get(4))); // 4
}
