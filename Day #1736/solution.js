// FIFO queue via two stacks (in/out); dequeue moves in->out when out empty. Amortized O(1) per op, O(n) space.
"use strict";

class MyQueue {
  constructor() {
    this.inStack = [];
    this.outStack = [];
  }
  enqueue(x) {
    this.inStack.push(x);
  }
  dequeue() {
    if (this.outStack.length === 0) {
      while (this.inStack.length > 0) this.outStack.push(this.inStack.pop());
    }
    return this.outStack.pop();
  }
}

function main() {
  const q = new MyQueue();
  q.enqueue(1); q.enqueue(2); q.enqueue(3);
  console.log(q.dequeue());
  q.enqueue(4);
  console.log(q.dequeue());
  console.log(q.dequeue());
  console.log(q.dequeue());
}

main();
