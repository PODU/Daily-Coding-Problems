// Day 369: Stock price tracker.
// ts2price maps timestamp->price; a BTreeMap<price,count> multiset gives min/max
// in O(log n), running sum+count give average. add/update/remove O(log n).
use std::collections::{BTreeMap, HashMap};

struct StockTracker {
    ts2price: HashMap<i64, i64>,
    prices: BTreeMap<i64, i64>, // price -> count
    sum: i64,
    count: i64,
}

impl StockTracker {
    fn new() -> Self {
        StockTracker { ts2price: HashMap::new(), prices: BTreeMap::new(), sum: 0, count: 0 }
    }
    fn add(&mut self, ts: i64, price: i64) {
        self.ts2price.insert(ts, price);
        *self.prices.entry(price).or_insert(0) += 1;
        self.sum += price;
        self.count += 1;
    }
    fn update(&mut self, ts: i64, price: i64) {
        self.remove(ts);
        self.add(ts, price);
    }
    fn remove(&mut self, ts: i64) {
        if let Some(price) = self.ts2price.remove(&ts) {
            if let Some(c) = self.prices.get_mut(&price) {
                *c -= 1;
                if *c == 0 {
                    self.prices.remove(&price);
                }
            }
            self.sum -= price;
            self.count -= 1;
        }
    }
    fn max(&self) -> i64 { *self.prices.keys().next_back().unwrap() }
    fn min(&self) -> i64 { *self.prices.keys().next().unwrap() }
    fn average(&self) -> f64 { self.sum as f64 / self.count as f64 }
}

fn main() {
    let mut s = StockTracker::new();
    s.add(1, 100); s.add(2, 200); s.add(3, 150);
    println!("max={} min={} avg={:.1}", s.max(), s.min(), s.average());
    s.update(2, 50);
    println!("max={} min={} avg={:.1}", s.max(), s.min(), s.average());
    s.remove(3);
    println!("max={} min={} avg={:.1}", s.max(), s.min(), s.average());
}
