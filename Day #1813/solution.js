// Last-N order log via fixed-size circular buffer.
// record: O(1), get_last(i): O(1). Space: O(N).
class OrderLog {
  constructor(n) {
    this.n = n;
    this.buf = new Array(n).fill(null);
    this.head = 0; // index where next write goes
    this.count = 0;
  }
  record(id) {
    this.buf[this.head] = id;
    this.head = (this.head + 1) % this.n;
    if (this.count < this.n) this.count++;
  }
  getLast(i) {
    return this.buf[((this.head - i) % this.n + this.n) % this.n];
  }
}

const log = new OrderLog(5);
for (const id of [101, 102, 103, 104, 105, 106]) log.record(id);
console.log(log.getLast(1)); // 106
console.log(log.getLast(3)); // 104
