// Day 1117 - Three stacks backed by a single list
// Each entry stores (value, prev_index); per-stack heads + free list give O(1)
// push/pop sharing one list. Space O(n).

struct Stack3 {
    list: Vec<(i32, i32)>, // (value, prev)
    heads: [i32; 3],
    free: Vec<usize>,
}

impl Stack3 {
    fn new() -> Self {
        Stack3 { list: Vec::new(), heads: [-1, -1, -1], free: Vec::new() }
    }
    fn push(&mut self, item: i32, s: usize) {
        let idx = if let Some(i) = self.free.pop() {
            self.list[i] = (item, self.heads[s]);
            i
        } else {
            self.list.push((item, self.heads[s]));
            self.list.len() - 1
        };
        self.heads[s] = idx as i32;
    }
    fn pop(&mut self, s: usize) -> i32 {
        let idx = self.heads[s];
        if idx == -1 {
            panic!("pop from empty stack {}", s);
        }
        let (value, prev) = self.list[idx as usize];
        self.heads[s] = prev;
        self.free.push(idx as usize);
        value
    }
}

fn main() {
    let mut s = Stack3::new();
    s.push(1, 0);
    s.push(2, 0);
    s.push(3, 1);
    s.push(4, 2);
    s.push(5, 2);
    println!("pop(0): {}", s.pop(0)); // 2
    println!("pop(0): {}", s.pop(0)); // 1
    println!("pop(2): {}", s.pop(2)); // 5
    println!("pop(1): {}", s.pop(1)); // 3
    println!("pop(2): {}", s.pop(2)); // 4
}
