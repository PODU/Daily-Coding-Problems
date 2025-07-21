// XOR linked list simulated with a "memory" Vec; addresses are indices (0 = null).
// each node stores both = prevAddr XOR nextAddr. add: O(1), get(i): O(i). Space: O(n).
struct Node {
    val: i32,
    both: usize, // prevAddr XOR nextAddr
}

struct XorList {
    mem: Vec<Option<Node>>,
    head: usize,
    tail: usize,
}

impl XorList {
    fn new() -> Self {
        XorList { mem: vec![None], head: 0, tail: 0 } // index 0 reserved as null
    }
    fn alloc(&mut self, n: Node) -> usize {
        self.mem.push(Some(n));
        self.mem.len() - 1
    }
    fn add(&mut self, val: i32) {
        let addr = self.alloc(Node { val, both: 0 });
        if self.head == 0 {
            self.head = addr;
            self.tail = addr;
            return;
        }
        let tail = self.tail;
        self.mem[tail].as_mut().unwrap().both ^= addr;
        self.mem[addr].as_mut().unwrap().both = tail;
        self.tail = addr;
    }
    fn get(&self, index: usize) -> Option<&Node> {
        let mut prev = 0usize;
        let mut cur = self.head;
        for _ in 0..index {
            if cur == 0 {
                break;
            }
            let next = self.mem[cur].as_ref().unwrap().both ^ prev;
            prev = cur;
            cur = next;
        }
        if cur == 0 { None } else { self.mem[cur].as_ref() }
    }
}

fn main() {
    let mut l = XorList::new();
    for v in [10, 20, 30, 40] {
        l.add(v);
    }
    let out: Vec<String> = (0..4).map(|i| l.get(i).unwrap().val.to_string()).collect();
    println!("{}", out.join(" "));
}
