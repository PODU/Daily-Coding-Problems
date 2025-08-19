// Three stacks in one list via interleaved indexing: logical pos p of stack s -> physical p*3+s.
// push/pop O(1) amortized. Space O(total elements). Single backing list.

struct Stack {
    list: Vec<i64>,    // single backing list
    sizes: [usize; 3], // logical height of each stack
}

impl Stack {
    fn new() -> Self {
        Stack { list: Vec::new(), sizes: [0, 0, 0] }
    }
    fn push(&mut self, item: i64, stack_number: usize) {
        let phys = self.sizes[stack_number] * 3 + stack_number;
        while self.list.len() <= phys {
            self.list.push(0);
        }
        self.list[phys] = item;
        self.sizes[stack_number] += 1;
    }
    fn pop(&mut self, stack_number: usize) -> i64 {
        assert!(self.sizes[stack_number] > 0, "empty stack");
        self.sizes[stack_number] -= 1;
        let phys = self.sizes[stack_number] * 3 + stack_number;
        self.list[phys]
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(1, 0);
    s.push(2, 0);
    s.push(10, 1);
    s.push(100, 2);
    s.push(200, 2);
    println!("{} {} {} {} {}", s.pop(0), s.pop(2), s.pop(1), s.pop(2), s.pop(0));
    // 2 200 10 100 1
}
