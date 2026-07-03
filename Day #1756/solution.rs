// Day 1756: HitCounter design.
// Store timestamps in a sorted Vec; total() O(1), range() via binary search O(log n).
// Limited-memory follow-up: bucket hits by coarse time granularity (e.g. per-second
// counts in a map/ring buffer) so memory is O(#buckets) instead of O(#hits).

struct HitCounter {
    hits: Vec<i64>, // kept sorted
}

impl HitCounter {
    fn new() -> Self {
        HitCounter { hits: Vec::new() }
    }
    fn record(&mut self, timestamp: i64) {
        let idx = self.hits.partition_point(|&x| x <= timestamp); // first > ts
        self.hits.insert(idx, timestamp);
    }
    fn total(&self) -> usize {
        self.hits.len()
    }
    fn range(&self, lower: i64, upper: i64) -> usize {
        let lo = self.hits.partition_point(|&x| x < lower);  // first >= lower
        let hi = self.hits.partition_point(|&x| x <= upper); // first > upper
        hi - lo
    }
}

fn main() {
    let mut hc = HitCounter::new();
    for t in [1, 2, 2, 5, 7, 9, 10] {
        hc.record(t);
    }

    println!("total() = {}", hc.total());           // 7
    println!("range(2, 7) = {}", hc.range(2, 7));    // 4
    println!("range(0, 10) = {}", hc.range(0, 10));  // 7
}
