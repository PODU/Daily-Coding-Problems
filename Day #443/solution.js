// Day 443: FIFO queue from two stacks. Amortized O(1) per op: push onto `inn`,
// pop from `out`, refilling `out` from `inn` when empty.

class QueueTwoStacks {
  constructor() { this.inn = []; this.out = []; }
  enqueue(x) { this.inn.push(x); }
  dequeue() {
    if (this.out.length === 0) {
      if (this.inn.length === 0) throw new Error("queue is empty");
      while (this.inn.length) this.out.push(this.inn.pop());
    }
    return this.out.pop();
  }
}

const q = new QueueTwoStacks();
q.enqueue(1); q.enqueue(2); q.enqueue(3);
console.log(q.dequeue()); // 1
console.log(q.dequeue()); // 2
q.enqueue(4);
console.log(q.dequeue()); // 3
console.log(q.dequeue()); // 4
