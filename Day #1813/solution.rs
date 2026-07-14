// Last-N order log via fixed-size circular buffer.
// record: O(1), get_last(i): O(1). Space: O(N).
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
        let idx = (self.head + self.n - i) % self.n;
        self.buf[idx]
    }
}

fn main() {
    let mut log = OrderLog::new(5);
    for id in [101i64, 102, 103, 104, 105, 106] {
        log.record(id);
    }
    println!("{}", log.get_last(1)); // 106
    println!("{}", log.get_last(3)); // 104
}
