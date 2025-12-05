// Day 696: 24-hour subscriber array with point update + inclusive range-sum query.
// Approach: Fenwick (Binary Indexed) Tree. update O(log n), query O(log n), space O(n).
struct Fenwick {
    t: Vec<i64>,
    n: usize,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { t: vec![0; n + 1], n }
    }
    fn add(&mut self, mut i: usize, v: i64) {
        i += 1;
        while i <= self.n {
            self.t[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn pref(&self, mut i: usize) -> i64 {
        i += 1;
        let mut s = 0;
        while i > 0 {
            s += self.t[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    fn range(&self, l: usize, r: usize) -> i64 {
        self.pref(r) - if l > 0 { self.pref(l - 1) } else { 0 }
    }
}

struct Subscribers {
    f: Fenwick,
}

impl Subscribers {
    fn new() -> Self { Subscribers { f: Fenwick::new(24) } }
    fn update(&mut self, hour: usize, value: i64) { self.f.add(hour, value); }
    fn query(&self, start: usize, end: usize) -> i64 { self.f.range(start, end) }
}

fn main() {
    let mut s = Subscribers::new();
    s.update(3, 10);
    s.update(5, 7);
    s.update(10, 4);
    println!("{}", s.query(3, 10)); // 21
    println!("{}", s.query(0, 4));  // 10
    s.update(3, 5);
    println!("{}", s.query(3, 10)); // 26
}
