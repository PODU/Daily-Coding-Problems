// Day 504: Last-N order log via fixed-size circular buffer.
// record O(1), get_last(i) O(1); space O(N).

struct OrderLog {
    buf: Vec<i64>,
    cap: usize,
    pos: usize,   // next write index
    count: usize, // records seen (capped at cap)
}

impl OrderLog {
    fn new(n: usize) -> Self {
        OrderLog {
            buf: vec![0; n],
            cap: n,
            pos: 0,
            count: 0,
        }
    }

    fn record(&mut self, order_id: i64) {
        self.buf[self.pos] = order_id;
        self.pos = (self.pos + 1) % self.cap;
        if self.count < self.cap {
            self.count += 1;
        }
    }

    // i = 1 is the most recent.
    fn get_last(&self, i: usize) -> i64 {
        let idx = (self.pos + self.cap - (i % self.cap)) % self.cap;
        self.buf[idx]
    }
}

fn main() {
    let mut log = OrderLog::new(5);
    for id in [1, 2, 3, 4, 5, 6, 7] {
        log.record(id);
    }
    println!("get_last(1) = {}", log.get_last(1));
    println!("get_last(2) = {}", log.get_last(2));
    println!("get_last(3) = {}", log.get_last(3));
}
