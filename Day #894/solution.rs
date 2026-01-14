// HitCounter: maintain timestamps in a sorted Vec via binary-search insert. record O(n) shift,
// total = len O(1), range = partition_point upper - lower via binary search O(log n).
// Limited-memory follow-up: bucket hits by time window (circular buffer of fixed size)
// so memory stays O(window) instead of O(#hits), trading exact old-range queries for recency.

struct HitCounter {
    ts: Vec<i64>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter { ts: Vec::new() }
    }

    fn record(&mut self, t: i64) {
        let pos = self.ts.partition_point(|&x| x < t);
        self.ts.insert(pos, t);
    }

    fn total(&self) -> usize {
        self.ts.len()
    }

    fn range(&self, lower: i64, upper: i64) -> usize {
        let lo = self.ts.partition_point(|&x| x < lower);
        let hi = self.ts.partition_point(|&x| x <= upper);
        hi - lo
    }
}

fn main() {
    let mut hc = HitCounter::new();
    hc.record(1);
    hc.record(2);
    hc.record(3);
    hc.record(2);
    println!("{}", hc.total());      // 4
    println!("{}", hc.range(2, 3));  // 3
}
