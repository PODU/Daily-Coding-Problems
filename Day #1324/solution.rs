// Day 1324: Point-update / range-sum over a 24-element array using a Fenwick (Binary Indexed) Tree.
// update O(log n), query O(log n). 1-indexed internally over fixed size 24.

struct Subscribers {
    tree: [i32; 25],
}

impl Subscribers {
    fn new() -> Self { Subscribers { tree: [0; 25] } }

    fn update(&mut self, hour: usize, value: i32) {
        let mut i = hour + 1;
        while i <= 24 {
            self.tree[i] += value;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, hour: usize) -> i32 {
        let mut s = 0;
        let mut i = hour + 1;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn query(&self, start: usize, end: usize) -> i32 {
        let left = if start > 0 { self.prefix(start - 1) } else { 0 };
        self.prefix(end) - left
    }
}

fn main() {
    let mut s = Subscribers::new();
    s.update(2, 10);
    s.update(5, 3);
    s.update(5, 7);
    println!("{}", s.query(2, 5));  // 20
    println!("{}", s.query(0, 23)); // 20
    println!("{}", s.query(3, 4));  // 0
}
