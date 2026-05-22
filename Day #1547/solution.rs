// XOR linked list simulated with vectors indexed by integer addresses.
// add appends in O(1); get traverses with next_addr = cur_both XOR prev_addr in O(index). Space O(n).

struct XorList {
    val: Vec<i32>,    // address 0 is sentinel/null
    both: Vec<usize>, // prev_addr XOR next_addr
    head: usize,
    tail: usize,
}

impl XorList {
    fn new() -> Self {
        XorList { val: vec![0], both: vec![0], head: 0, tail: 0 }
    }
    fn add(&mut self, v: i32) {
        let addr = self.val.len();
        self.val.push(v);
        self.both.push(0);
        if self.head == 0 {
            self.head = addr;
            self.tail = addr;
        } else {
            self.both[self.tail] ^= addr;
            self.both[addr] ^= self.tail;
            self.tail = addr;
        }
    }
    fn get(&self, index: usize) -> i32 {
        let mut prev = 0usize;
        let mut cur = self.head;
        for _ in 0..index {
            let next = self.both[cur] ^ prev;
            prev = cur;
            cur = next;
        }
        self.val[cur]
    }
}

fn main() {
    let mut list = XorList::new();
    for v in [10, 20, 30, 40, 50] {
        list.add(v);
    }
    println!("{}", list.get(0));
    println!("{}", list.get(2));
    println!("{}", list.get(4));
}
