// Day 1295: Fixed-size log of last N order ids via circular buffer.
// record and getLast are both O(1) time, O(N) space.
class OrderLog {
  constructor(n) {
    this.n = n;
    this.buf = new Array(n);
    this.head = 0; // index of next write
    this.count = 0;
  }

  record(orderId) {
    this.buf[this.head] = orderId;
    this.head = (this.head + 1) % this.n;
    if (this.count < this.n) this.count++;
  }

  getLast(i) {
    // 1 = most recent
    return this.buf[((this.head - i) % this.n + this.n) % this.n];
  }
}

const log = new OrderLog(3);
log.record(10);
log.record(20);
log.record(30);
console.log(log.getLast(1)); // 30
console.log(log.getLast(2)); // 20
log.record(40);
console.log(log.getLast(1)); // 40
console.log(log.getLast(3)); // 20
