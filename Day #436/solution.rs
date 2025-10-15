// Day 436: Three stacks backed by one Vec using node-based singly-linked
// indices + a free list. push/pop are O(1) time, O(n) space overall.

struct ThreeStacks {
    data: Vec<(i64, i64)>, // (val, prev_index)
    tops: [i64; 3],
    free: Vec<usize>,
}

impl ThreeStacks {
    fn new() -> Self {
        ThreeStacks { data: Vec::new(), tops: [-1, -1, -1], free: Vec::new() }
    }
    fn push(&mut self, item: i64, s: usize) {
        let idx = if let Some(i) = self.free.pop() {
            self.data[i] = (item, self.tops[s]);
            i
        } else {
            self.data.push((item, self.tops[s]));
            self.data.len() - 1
        };
        self.tops[s] = idx as i64;
    }
    fn pop(&mut self, s: usize) -> i64 {
        let idx = self.tops[s];
        if idx == -1 {
            panic!("stack {} is empty", s);
        }
        let (val, prev) = self.data[idx as usize];
        self.tops[s] = prev;
        self.free.push(idx as usize);
        val
    }
}

fn main() {
    let mut st = ThreeStacks::new();
    st.push(1, 0); st.push(2, 0);
    st.push(10, 1);
    st.push(100, 2); st.push(200, 2);
    println!("{}", st.pop(0)); // 2
    println!("{}", st.pop(0)); // 1
    println!("{}", st.pop(1)); // 10
    println!("{}", st.pop(2)); // 200
    println!("{}", st.pop(2)); // 100
}
