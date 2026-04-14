// XOR linked list using real raw-pointer XOR (usize). add=O(1), get(i)=O(i) time, O(1) extra space.
struct Node {
    val: i32,
    both: usize, // XOR of prev and next addresses
}

struct XorList {
    head: usize, // raw pointer as usize, 0 = null
    tail: usize,
}

impl XorList {
    fn new() -> Self { XorList { head: 0, tail: 0 } }

    fn add(&mut self, v: i32) {
        let n = Box::into_raw(Box::new(Node { val: v, both: 0 })) as usize;
        if self.head == 0 {
            self.head = n;
            self.tail = n;
            return;
        }
        unsafe {
            (*(n as *mut Node)).both = self.tail;        // prev=tail, next=null(0)
            (*(self.tail as *mut Node)).both ^= n;       // append n as next of old tail
        }
        self.tail = n;
    }

    fn get(&self, index: usize) -> i32 {
        let mut prev: usize = 0;
        let mut cur: usize = self.head;
        unsafe {
            for _ in 0..index {
                let next = (*(cur as *const Node)).both ^ prev;
                prev = cur;
                cur = next;
            }
            (*(cur as *const Node)).val
        }
    }
}

fn main() {
    let mut list = XorList::new();
    list.add(10); list.add(20); list.add(30); list.add(40);
    println!("{}", list.get(0));
    println!("{}", list.get(1));
    println!("{}", list.get(2));
    println!("{}", list.get(3));
}
