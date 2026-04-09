// Day 1327: Queue backed by a list of fixed-length blocks (chunks).
// enqueue appends to the tail block (new block when full); dequeue pops from the head block. Amortized O(1).

class BlockQueue {
  constructor() {
    this.BLOCK = 4;
    this.blocks = [];
    this.head = 0;   // read index in the front block
    this.size = 0;
  }
  enqueue(x) {
    if (this.blocks.length === 0 || this.blocks[this.blocks.length - 1].length === this.BLOCK) {
      this.blocks.push([]);
    }
    this.blocks[this.blocks.length - 1].push(x);
    this.size++;
  }
  dequeue() {
    if (this.size === 0) throw new Error("empty");
    const x = this.blocks[0][this.head++];
    this.size--;
    if (this.head === this.blocks[0].length) {
      this.blocks.shift();
      this.head = 0;
    }
    return x;
  }
  getSize() { return this.size; }
}

const q = new BlockQueue();
for (let i = 1; i <= 5; i++) q.enqueue(i);
console.log(q.dequeue()); // 1
console.log(q.dequeue()); // 2
console.log(q.getSize()); // 3
