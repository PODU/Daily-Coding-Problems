// Day 663: HitCounter. Keep timestamps sorted; total = O(1); range = binary search.
// record O(n) sorted insert, total O(1), range O(log n).
// Limited-memory follow-up: bucket hits into fixed time windows storing (window, count).
struct HitCounter {
    ts: Vec<i64>,
}

impl HitCounter {
    fn new() -> Self { HitCounter { ts: Vec::new() } }
    fn record(&mut self, t: i64) {
        let i = self.ts.partition_point(|&x| x < t);
        self.ts.insert(i, t);
    }
    fn total(&self) -> usize { self.ts.len() }
    fn range(&self, lo: i64, hi: i64) -> usize {
        let l = self.ts.partition_point(|&x| x < lo);
        let r = self.ts.partition_point(|&x| x <= hi);
        r - l
    }
}

fn main() {
    let mut h = HitCounter::new();
    for t in [1, 2, 2, 5, 9, 10] {
        h.record(t);
    }
    println!("total: {}", h.total());          // 6
    println!("range(2,9): {}", h.range(2, 9));  // 4
}
