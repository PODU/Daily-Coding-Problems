// Max stack with O(1) push/pop/max.
// Auxiliary stack stores running maxima alongside main stack. All ops O(1); space O(N).

struct MaxStack {
    data: Vec<i32>,
    maxs: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack { data: Vec::new(), maxs: Vec::new() }
    }
    fn push(&mut self, val: i32) {
        let m = match self.maxs.last() {
            Some(&top) => top.max(val),
            None => val,
        };
        self.data.push(val);
        self.maxs.push(m);
    }
    // returns top, or None if empty
    fn pop(&mut self) -> Option<i32> {
        self.maxs.pop();
        self.data.pop()
    }
    fn max(&self) -> Option<i32> {
        self.maxs.last().copied()
    }
}

fn show(v: Option<i32>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let mut s = MaxStack::new();
    s.push(3);
    s.push(1);
    s.push(5);
    s.push(2);
    println!("max: {}", show(s.max()));
    println!("pop: {}", show(s.pop()));
    println!("pop: {}", show(s.pop()));
    println!("max: {}", show(s.max()));
}
