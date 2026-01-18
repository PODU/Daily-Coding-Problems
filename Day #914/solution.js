// Queue via fixed-length array blocks (capacity 4). List of blocks; enqueue to tail, dequeue from head.
// Amortized O(1) per op; O(n) space.
class BlockQueue {
  static CAP = 4;
  constructor() {
    this.blocks = [];
    this.headIdx = 0;
    this.tailCount = 0;
    this.size = 0;
  }
  enqueue(v) {
    if (this.blocks.length === 0 || this.tailCount === BlockQueue.CAP) {
      this.blocks.push(new Array(BlockQueue.CAP).fill(0));
      this.tailCount = 0;
    }
    this.blocks[this.blocks.length - 1][this.tailCount++] = v;
    this.size++;
  }
  dequeue() {
    if (this.size === 0) throw new Error("empty");
    const v = this.blocks[0][this.headIdx++];
    this.size--;
    if (this.headIdx === BlockQueue.CAP || (this.blocks.length === 1 && this.headIdx === this.tailCount)) {
      this.blocks.shift();
      this.headIdx = 0;
      if (this.blocks.length === 0) this.tailCount = 0;
    }
    return v;
  }
  getSize() {
    return this.size;
  }
}

function main() {
  const q = new BlockQueue();
  for (let i = 1; i <= 10; i++) q.enqueue(i);
  const dq = [];
  for (let i = 0; i < 3; i++) dq.push(q.dequeue());
  console.log("Dequeued: " + dq.join(" "));
  console.log("Size: " + q.getSize());
}

main();
