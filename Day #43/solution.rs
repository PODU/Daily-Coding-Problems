// Max Stack: main stack + auxiliary stack holding running max. All ops O(1).
// Time O(1) per op; Space O(n).

struct MaxStack {
    data: Vec<i32>,
    maxes: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack { data: Vec::new(), maxes: Vec::new() }
    }
    fn push(&mut self, v: i32) {
        self.data.push(v);
        let m = match self.maxes.last() {
            Some(&cur) => cur.max(v),
            None => v,
        };
        self.maxes.push(m);
    }
    fn pop(&mut self) -> Option<i32> {
        self.maxes.pop();
        self.data.pop()
    }
    fn max(&self) -> Option<i32> {
        self.maxes.last().copied()
    }
}

fn show(o: Option<i32>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let mut s = MaxStack::new();
    for v in [3, 1, 5, 4] {
        s.push(v);
        println!("push {} -> max={}", v, show(s.max()));
    }
    println!("pop -> {}, max={}", show(s.pop()), show(s.max()));
    println!("pop -> {}, max={}", show(s.pop()), show(s.max()));
}
