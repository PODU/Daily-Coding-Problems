// Day 504: Last-N order log via fixed-size circular buffer.
// record O(1), get_last(i) O(1); space O(N).

class OrderLog {
  constructor(n) {
    this.cap = n;
    this.buf = new Array(n).fill(null);
    this.pos = 0; // next write index
    this.count = 0; // records seen (capped at cap)
  }

  record(orderId) {
    this.buf[this.pos] = orderId;
    this.pos = (this.pos + 1) % this.cap;
    if (this.count < this.cap) this.count++;
  }

  // i = 1 is the most recent.
  getLast(i) {
    const idx = ((this.pos - i) % this.cap + this.cap) % this.cap;
    return this.buf[idx];
  }
}

function main() {
  const log = new OrderLog(5);
  for (const id of [1, 2, 3, 4, 5, 6, 7]) log.record(id);
  console.log("get_last(1) = " + log.getLast(1));
  console.log("get_last(2) = " + log.getLast(2));
  console.log("get_last(3) = " + log.getLast(3));
}

main();
