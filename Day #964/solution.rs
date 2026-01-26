// Day 964: Order log keeping last N ids with O(1) record/get_last.
// Approach: fixed-size circular buffer. Time O(1) per op, Space O(N).

struct OrderLog {
    buf: Vec<i32>,
    n: usize,
    count: usize,
}

impl OrderLog {
    fn new(n: usize) -> Self {
        OrderLog { buf: vec![0; n], n, count: 0 }
    }
    fn record(&mut self, order_id: i32) {
        self.buf[self.count % self.n] = order_id;
        self.count += 1;
    }
    fn get_last(&self, i: usize) -> i32 {
        self.buf[(self.count - i) % self.n]
    }
}

fn main() {
    let mut log = OrderLog::new(3);
    for x in [10, 20, 30, 40] { // 40 evicts 10
        log.record(x);
    }
    println!("{}", log.get_last(1)); // 40
    println!("{}", log.get_last(2)); // 30
    println!("{}", log.get_last(3)); // 20
}
