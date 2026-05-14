// Three stacks sharing ONE backing Vec of nodes (value, prevIndex) + free list.
// Three head pointers index into the single shared list. O(1) push/pop, O(n) space.

struct ThreeStacks {
    val: Vec<i32>,
    prev: Vec<i32>,
    head: [i32; 3],
    free_head: i32,
}

impl ThreeStacks {
    fn new() -> Self {
        ThreeStacks { val: Vec::new(), prev: Vec::new(), head: [-1, -1, -1], free_head: -1 }
    }

    fn alloc(&mut self, v: i32, p: i32) -> i32 {
        if self.free_head != -1 {
            let idx = self.free_head;
            self.free_head = self.prev[idx as usize];
            self.val[idx as usize] = v;
            self.prev[idx as usize] = p;
            idx
        } else {
            let idx = self.val.len() as i32;
            self.val.push(v);
            self.prev.push(p);
            idx
        }
    }

    fn push(&mut self, item: i32, s: usize) {
        let h = self.head[s];
        self.head[s] = self.alloc(item, h);
    }

    fn pop(&mut self, s: usize) -> i32 {
        let idx = self.head[s];
        let v = self.val[idx as usize];
        self.head[s] = self.prev[idx as usize];
        self.prev[idx as usize] = self.free_head;
        self.free_head = idx;
        v
    }
}

fn main() {
    let mut st = ThreeStacks::new();
    st.push(1, 0);
    st.push(2, 0);
    st.push(3, 1);
    st.push(4, 2);
    st.push(5, 2);
    println!("{} {} {} {}", st.pop(0), st.pop(2), st.pop(1), st.pop(0));
}
