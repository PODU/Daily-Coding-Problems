// Approach: Circular (ring) buffer of size N. record/get_last are O(1); O(N) space.

class OrderLog {
  constructor(n) {
    this.buf = new Array(n);
    this.n = n;
    this.count = 0;
    this.head = 0; // next write position
  }
  record(orderId) {
    this.buf[this.head] = orderId;
    this.head = (this.head + 1) % this.n;
    if (this.count < this.n) this.count++;
  }
  // i is 1-based: get_last(1) is the most recent
  getLast(i) {
    return this.buf[((this.head - i) % this.n + this.n) % this.n];
  }
}

const log = new OrderLog(3);
for (const x of [1, 2, 3, 4, 5]) log.record(x);
console.log(`get_last(1) = ${log.getLast(1)}`);
console.log(`get_last(2) = ${log.getLast(2)}`);
console.log(`get_last(3) = ${log.getLast(3)}`);
