// Day 1295: Fixed-size log of last N order ids via circular buffer.
// record and get_last are both O(1) time, O(N) space.
struct OrderLog {
    buf: Vec<i64>,
    n: usize,
    head: usize,
    count: usize,
}

impl OrderLog {
    fn new(n: usize) -> Self {
        OrderLog { buf: vec![0; n], n, head: 0, count: 0 }
    }

    fn record(&mut self, id: i64) {
        self.buf[self.head] = id;
        self.head = (self.head + 1) % self.n;
        if self.count < self.n {
            self.count += 1;
        }
    }

    fn get_last(&self, i: usize) -> i64 {
        // 1 = most recent
        let idx = (self.head + self.n - (i % self.n)) % self.n;
        self.buf[idx]
    }
}

fn main() {
    let mut log = OrderLog::new(3);
    log.record(10);
    log.record(20);
    log.record(30);
    println!("{}", log.get_last(1)); // 30
    println!("{}", log.get_last(2)); // 20
    log.record(40);
    println!("{}", log.get_last(1)); // 40
    println!("{}", log.get_last(3)); // 20
}
