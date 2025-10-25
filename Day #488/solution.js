// Day 488: Queue backed by a set of fixed-length arrays (blocks).
// Blocks of size B chained; head/tail indices roll over to next block.
// enqueue/dequeue amortized O(1), get_size O(1). Space O(n).
class BlockQueue {
  constructor() {
    this.B = 4;        // fixed block length
    this.blocks = [];  // set of fixed-length arrays
    this.head = 0;
    this.tail = 0;
    this.sz = 0;
  }

  enqueue(x) {
    if (this.blocks.length === 0 || this.tail === this.B) { // allocate new block
      this.blocks.push(new Array(this.B).fill(0));
      this.tail = 0;
    }
    this.blocks[this.blocks.length - 1][this.tail++] = x;
    this.sz++;
  }

  dequeue() {
    if (this.sz === 0) throw new Error("empty");
    const x = this.blocks[0][this.head++];
    this.sz--;
    if (this.head === this.B) {     // front block exhausted -> free it
      this.blocks.shift();
      this.head = 0;
    }
    if (this.blocks.length === 1 && this.head === this.tail) { // single block consumed
      this.blocks = [];
      this.head = this.tail = 0;
    }
    return x;
  }

  getSize() {
    return this.sz;
  }
}

const q = new BlockQueue();
for (let i = 1; i <= 6; i++) q.enqueue(i); // 1..6
console.log("size=" + q.getSize()); // 6
console.log("deq=" + q.dequeue());   // 1
console.log("deq=" + q.dequeue());   // 2
q.enqueue(7);
q.enqueue(8);
console.log("size=" + q.getSize()); // 6
const out = [];
while (q.getSize() > 0) out.push(q.dequeue()); // 3 4 5 6 7 8
console.log(out.join(" "));
