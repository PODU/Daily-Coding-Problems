// Queue built from a list of fixed-capacity blocks (cap 4). Track head/tail block+offset and an O(1) size.
// enqueue/dequeue/get_size all amortized O(1) time; O(n) space.
"use strict";

class BlockQueue {
  constructor() {
    this.CAP = 4;
    this.blocks = [];
    this.headBlock = 0; // index of front block
    this.headOff = 0;   // offset within front block
    this.tailOff = 0;   // next write offset within back block
    this.sz = 0;
  }

  enqueue(v) {
    if (this.blocks.length === 0 || this.tailOff === this.CAP) {
      this.blocks.push(new Array(this.CAP));
      this.tailOff = 0;
    }
    this.blocks[this.blocks.length - 1][this.tailOff++] = v;
    this.sz++;
  }

  dequeue() {
    const v = this.blocks[this.headBlock][this.headOff++];
    this.sz--;
    if (this.headOff === this.CAP) { // front block fully consumed
      this.headBlock++;
      this.headOff = 0;
    }
    return v;
  }

  get_size() {
    return this.sz;
  }
}

function main() {
  const q = new BlockQueue();
  for (const v of [1, 2, 3, 4, 5]) q.enqueue(v);
  console.log("size=" + q.get_size());
  console.log(q.dequeue());
  console.log(q.dequeue());
  console.log(q.dequeue());
  console.log("size=" + q.get_size());
}

main();
