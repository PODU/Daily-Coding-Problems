// Three stacks in one array via fixed equal regions, each with its own top. O(1) push/pop.
struct ThreeStacks {
    cap: usize,
    arr: Vec<i32>,
    top: [usize; 3],
}

impl ThreeStacks {
    fn new(per_stack: usize) -> Self {
        ThreeStacks { cap: per_stack, arr: vec![0; 3 * per_stack], top: [0; 3] }
    }
    fn push(&mut self, item: i32, sn: usize) {
        if self.top[sn] >= self.cap {
            panic!("stack full");
        }
        self.arr[sn * self.cap + self.top[sn]] = item;
        self.top[sn] += 1;
    }
    fn pop(&mut self, sn: usize) -> i32 {
        if self.top[sn] == 0 {
            panic!("stack empty");
        }
        self.top[sn] -= 1;
        self.arr[sn * self.cap + self.top[sn]]
    }
}

fn main() {
    let mut s = ThreeStacks::new(3);
    s.push(1, 0);
    s.push(2, 0);
    s.push(10, 1);
    s.push(20, 1);
    s.push(100, 2);
    println!("stack0 pop: {}", s.pop(0));
    println!("stack1 pop: {}", s.pop(1));
    println!("stack2 pop: {}", s.pop(2));
    println!("stack0 pop: {}", s.pop(0));
}
