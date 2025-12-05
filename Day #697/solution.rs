// Day 697: LRU cache with O(1) get/set.
// Approach: hash map (key -> node index) + intrusive doubly linked list in a Vec
// with a free list. get/set O(1) time, O(n) space.
use std::collections::HashMap;

struct NodeL {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    cap: usize,
    nodes: Vec<NodeL>,
    head: usize, // sentinel MRU side
    tail: usize, // sentinel LRU side
    map: HashMap<i32, usize>,
    free: Vec<usize>,
}

impl LRUCache {
    fn new(n: usize) -> Self {
        // index 0 = head sentinel, 1 = tail sentinel
        let mut nodes = Vec::new();
        nodes.push(NodeL { key: 0, val: 0, prev: 0, next: 1 });
        nodes.push(NodeL { key: 0, val: 0, prev: 0, next: 1 });
        LRUCache { cap: n, nodes, head: 0, tail: 1, map: HashMap::new(), free: Vec::new() }
    }
    fn unlink(&mut self, i: usize) {
        let (p, nx) = (self.nodes[i].prev, self.nodes[i].next);
        self.nodes[p].next = nx;
        self.nodes[nx].prev = p;
    }
    fn push_front(&mut self, i: usize) {
        let after = self.nodes[self.head].next;
        self.nodes[i].prev = self.head;
        self.nodes[i].next = after;
        self.nodes[self.head].next = i;
        self.nodes[after].prev = i;
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
    fn set(&mut self, key: i32, value: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = value;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        if self.map.len() == self.cap {
            let lru = self.nodes[self.tail].prev;
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.free.push(lru);
        }
        let idx = match self.free.pop() {
            Some(f) => {
                self.nodes[f] = NodeL { key, val: value, prev: 0, next: 0 };
                f
            }
            None => {
                self.nodes.push(NodeL { key, val: value, prev: 0, next: 0 });
                self.nodes.len() - 1
            }
        };
        self.push_front(idx);
        self.map.insert(key, idx);
    }
}

fn main() {
    let mut c = LRUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    println!("{}", c.get(1).unwrap()); // 1
    c.set(3, 3); // evicts key 2
    match c.get(2) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
    println!("{}", c.get(3).unwrap()); // 3
}
