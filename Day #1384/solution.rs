// HitCounter: keep timestamps in a sorted Vec (binary-insert); record O(n), total O(1), range via binary search (upper-lower).
struct HitCounter {
    hits: Vec<i32>,
    cnt: u64,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter { hits: Vec::new(), cnt: 0 }
    }

    fn record(&mut self, t: i32) {
        let i = self.hits.partition_point(|&x| x < t);
        self.hits.insert(i, t);
        self.cnt += 1;
    }

    fn total(&self) -> u64 {
        self.cnt
    }

    fn range(&self, lo: i32, hi: i32) -> usize {
        let left = self.hits.partition_point(|&x| x < lo);
        let right = self.hits.partition_point(|&x| x <= hi);
        right - left
    }
}

fn main() {
    let mut hc = HitCounter::new();
    for t in [1, 1, 2, 3, 5, 8] {
        hc.record(t);
    }
    println!("total: {}", hc.total());
    println!("range(2,5): {}", hc.range(2, 5));
}
