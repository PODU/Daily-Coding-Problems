// Fenwick/Binary Indexed Tree over 24 hours.
// update: O(log n), query (prefix-diff): O(log n). Space O(n).
struct BIT {
    n: usize,
    tree: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT { n, tree: vec![0; n + 1] }
    }
    fn add(&mut self, i: usize, v: i64) {   // 0-based index
        let mut i = i + 1;
        while i <= self.n {
            self.tree[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, i: usize) -> i64 {     // sum of [0..i], 0-based
        let mut i = i + 1;
        let mut s = 0i64;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    fn query(&self, l: usize, r: usize) -> i64 { // inclusive [l..r]
        let left = if l > 0 { self.prefix(l - 1) } else { 0 };
        self.prefix(r) - left
    }
    fn update(&mut self, hour: usize, value: i64) {
        self.add(hour, value);
    }
}

fn main() {
    let mut bit = BIT::new(24);
    bit.update(2, 5);
    bit.update(5, 3);
    bit.update(23, 10);
    println!("query(2,5) = {}", bit.query(2, 5));
    println!("query(0,23) = {}", bit.query(0, 23));
}
