// Unrolled/paged queue: list of fixed-length blocks; head/tail indices. Amortized O(1) per op.
class BlockQueue {
  static BS = 4;
  constructor() {
    this.blocks = [];
    this.head = 0;
    this.tail = 0;
    this.sz = 0;
  }
  enqueue(x) {
    const BS = BlockQueue.BS;
    if (this.blocks.length === 0 || this.tail === BS) {
      this.blocks.push(new Array(BS).fill(0));
      this.tail = 0;
    }
    this.blocks[this.blocks.length - 1][this.tail++] = x;
    this.sz++;
  }
  dequeue() {
    if (this.sz === 0) throw new Error("empty");
    const x = this.blocks[0][this.head++];
    this.sz--;
    if (this.head === BlockQueue.BS || (this.blocks.length === 1 && this.head === this.tail)) {
      this.blocks.shift();
      this.head = 0;
      if (this.blocks.length === 0) this.tail = 0;
    }
    return x;
  }
  getSize() { return this.sz; }
  numBlocks() { return this.blocks.length; }
}

const q = new BlockQueue();
for (let i = 1; i <= 10; i++) q.enqueue(i);
console.log("size after enqueue 1..10:", q.getSize());
console.log("blocks allocated:", q.numBlocks());
console.log("dequeue 3:", q.dequeue(), q.dequeue(), q.dequeue());
console.log("size:", q.getSize());
q.enqueue(11);
console.log("enqueue 11, size:", q.getSize());
const rest = [];
while (q.getSize() > 0) rest.push(q.dequeue());
console.log("dequeue rest:", rest.join(" "));
console.log("size:", q.getSize());
