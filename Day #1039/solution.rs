// Fenwick/BIT over 24 hours: point update, prefix-sum range query.
// update O(log n), query O(log n).

struct BIT {
    n: usize,
    tree: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT { n, tree: vec![0; n + 1] }
    }
    fn update(&mut self, hour: usize, value: i64) {
        let mut i = hour + 1;
        while i <= self.n {
            self.tree[i] += value;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, idx: usize) -> i64 { // sum of [0..idx]
        let mut s = 0;
        let mut i = idx + 1;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    fn query(&self, start: usize, end: usize) -> i64 { // inclusive
        if start > 0 {
            self.prefix(end) - self.prefix(start - 1)
        } else {
            self.prefix(end)
        }
    }
}

fn main() {
    let mut bit = BIT::new(24);
    bit.update(0, 5);
    bit.update(3, 10);
    bit.update(23, 2);
    bit.update(3, 1);
    println!("query(0, 3) = {}", bit.query(0, 3));
    println!("query(0, 23) = {}", bit.query(0, 23));
    println!("query(4, 23) = {}", bit.query(4, 23));
    println!("query(3, 3) = {}", bit.query(3, 3));
}
