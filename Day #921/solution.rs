// XOR linked list via memory-pool simulation: nodes stored in a Vec, indices act as
// "addresses"; both = prevId XOR nextId (sentinel 0 = null, real nodes start at 1).
// add O(1) with tail tracking, get O(index). O(1) extra per node.

struct Node {
    value: i32,
    both: usize, // prevId XOR nextId
}

struct XorList {
    pool: Vec<Option<Node>>, // index 0 reserved as null sentinel
    head: usize,
    tail: usize,
}

impl XorList {
    fn new() -> Self {
        XorList { pool: vec![None], head: 0, tail: 0 }
    }

    fn add(&mut self, element: i32) {
        self.pool.push(Some(Node { value: element, both: 0 }));
        let id = self.pool.len() - 1;
        if self.head == 0 {
            self.head = id;
            self.tail = id;
        } else {
            let t = self.tail;
            self.pool[t].as_mut().unwrap().both ^= id; // old tail next becomes id
            self.pool[id].as_mut().unwrap().both = t;  // prev = old tail, next = 0
            self.tail = id;
        }
    }

    fn get(&self, index: usize) -> i32 {
        let mut prev = 0usize;
        let mut cur = self.head;
        for _ in 0..index {
            if cur == 0 {
                break;
            }
            let next = self.pool[cur].as_ref().unwrap().both ^ prev;
            prev = cur;
            cur = next;
        }
        if cur == 0 {
            panic!("index out of range");
        }
        self.pool[cur].as_ref().unwrap().value
    }
}

fn main() {
    let mut list = XorList::new();
    for v in [10, 20, 30, 40, 50] {
        list.add(v);
    }
    println!("get(0) = {}", list.get(0));
    println!("get(2) = {}", list.get(2));
    println!("get(4) = {}", list.get(4));
}
