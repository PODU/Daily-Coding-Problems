// Day 53: FIFO queue from two stacks. Amortized O(1) per op.
// in-stack receives pushes; out-stack serves pops (refilled when empty).
class Queue {
  constructor() {
    this.inStack = [];
    this.outStack = [];
  }

  enqueue(x) { this.inStack.push(x); }

  dequeue() {
    if (this.outStack.length === 0) {
      while (this.inStack.length) this.outStack.push(this.inStack.pop());
    }
    return this.outStack.pop();
  }
}

const q = new Queue();
q.enqueue(1);
q.enqueue(2);
q.enqueue(3);
console.log(q.dequeue()); // 1
console.log(q.dequeue()); // 2
q.enqueue(4);
console.log(q.dequeue()); // 3
console.log(q.dequeue()); // 4
