// XOR doubly linked list using a safe arena (Vec<Node>); address = arena index.
// index 0 is the null sentinel; both = prevIndex XOR nextIndex.
// add: O(1), get(i): O(i). Space O(n).

struct Node {
    value: i32,
    both: usize, // prevIndex XOR nextIndex
}

struct XorList {
    memory: Vec<Option<Node>>, // index 0 = null sentinel
    head: usize,
    tail: usize,
}

impl XorList {
    fn new() -> Self {
        XorList {
            memory: vec![None], // reserve index 0
            head: 0,
            tail: 0,
        }
    }

    fn add(&mut self, element: i32) {
        self.memory.push(Some(Node { value: element, both: 0 }));
        let idx = self.memory.len() - 1;
        if self.head == 0 {
            self.head = idx;
            self.tail = idx;
            return;
        }
        let tail = self.tail;
        if let Some(node) = self.memory[idx].as_mut() {
            node.both = tail ^ 0;
        }
        let tail_both = self.memory[tail].as_ref().unwrap().both;
        if let Some(tn) = self.memory[tail].as_mut() {
            tn.both = (tail_both ^ 0) ^ idx;
        }
        self.tail = idx;
    }

    fn get(&self, index: usize) -> i32 {
        let mut prev = 0usize;
        let mut cur = self.head;
        for _ in 0..index {
            let next = prev ^ self.memory[cur].as_ref().unwrap().both;
            prev = cur;
            cur = next;
        }
        self.memory[cur].as_ref().unwrap().value
    }
}

fn main() {
    let mut list = XorList::new();
    list.add(10);
    list.add(20);
    list.add(30);
    list.add(40);
    println!("{}", list.get(0));
    println!("{}", list.get(3));
}
