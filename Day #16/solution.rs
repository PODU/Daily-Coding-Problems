// Approach: Circular (ring) buffer of size N. record/get_last are O(1); O(N) space.

struct OrderLog {
    buf: Vec<i64>,
    n: usize,
    count: usize,
    head: usize, // next write position
}

impl OrderLog {
    fn new(n: usize) -> Self {
        OrderLog { buf: vec![0; n], n, count: 0, head: 0 }
    }
    fn record(&mut self, order_id: i64) {
        self.buf[self.head] = order_id;
        self.head = (self.head + 1) % self.n;
        if self.count < self.n {
            self.count += 1;
        }
    }
    // i is 1-based: get_last(1) is the most recent
    fn get_last(&self, i: usize) -> i64 {
        let idx = (self.head + self.n - i) % self.n;
        self.buf[idx]
    }
}

fn main() {
    let mut log = OrderLog::new(3);
    for x in [1, 2, 3, 4, 5] {
        log.record(x);
    }
    println!("get_last(1) = {}", log.get_last(1));
    println!("get_last(2) = {}", log.get_last(2));
    println!("get_last(3) = {}", log.get_last(3));
}
