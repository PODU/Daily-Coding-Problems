// Subscribers-per-hour over 24 hours via Fenwick/BIT. update(hour,val)+=, query(start,end)=inclusive range sum.
// Time O(log n) per op, Space O(n).

struct Fenwick {
    n: usize,
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { n, tree: vec![0; n + 1] }
    }
    fn update(&mut self, i: usize, v: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.tree[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn pref(&self, i: usize) -> i64 {
        let mut i = i + 1;
        let mut s = 0;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    fn query(&self, l: usize, r: usize) -> i64 {
        if l > 0 {
            self.pref(r) - self.pref(l - 1)
        } else {
            self.pref(r)
        }
    }
}

fn main() {
    let mut bit = Fenwick::new(24); // all zeros
    bit.update(0, 5);
    bit.update(3, 10);
    bit.update(23, 2);
    println!("{}", bit.query(0, 23)); // 17
    println!("{}", bit.query(0, 3));  // 15
    println!("{}", bit.query(1, 2));  // 0
}
