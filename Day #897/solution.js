// Queue via two stacks: enqueue->inStack; dequeue moves all to outStack when empty.
// Amortized O(1) per op, Space O(n).
class Queue {
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

const q = new Queue();
q.enqueue(1);
q.enqueue(2);
console.log(q.dequeue());
q.enqueue(3);
console.log(q.dequeue());
console.log(q.dequeue());
