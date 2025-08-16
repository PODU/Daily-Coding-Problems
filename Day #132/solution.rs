// Day 132: HitCounter (record, total, range).
// Keep timestamps sorted; range via partition_point. total O(1), range O(log n).
// Limited-memory follow-up: bucket counts by coarse time granularity instead of per-hit.
struct HitCounter {
    ts: Vec<i64>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter { ts: Vec::new() }
    }
    fn record(&mut self, t: i64) {
        let i = self.ts.partition_point(|&x| x <= t); // upper bound
        self.ts.insert(i, t);
    }
    fn total(&self) -> usize {
        self.ts.len()
    }
    fn range(&self, lo: i64, hi: i64) -> usize {
        let a = self.ts.partition_point(|&x| x < lo);
        let b = self.ts.partition_point(|&x| x <= hi);
        b - a
    }
}

fn main() {
    let mut hc = HitCounter::new();
    for t in [1, 1, 2, 3, 5, 8, 8, 10] {
        hc.record(t);
    }
    println!("total = {}", hc.total());          // 8
    println!("range(2, 8) = {}", hc.range(2, 8)); // 5
}
