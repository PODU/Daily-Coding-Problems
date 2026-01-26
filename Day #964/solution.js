// Day 964: Order log keeping last N ids with O(1) record/get_last.
// Approach: fixed-size circular buffer. Time O(1) per op, Space O(N).

class OrderLog {
  constructor(n) { this.buf = new Array(n); this.n = n; this.count = 0; }
  record(orderId) { this.buf[this.count % this.n] = orderId; this.count++; }
  getLast(i) { return this.buf[(((this.count - i) % this.n) + this.n) % this.n]; }
}

const log = new OrderLog(3);
[10, 20, 30, 40].forEach((x) => log.record(x)); // 40 evicts 10
console.log(log.getLast(1)); // 40
console.log(log.getLast(2)); // 30
console.log(log.getLast(3)); // 20
