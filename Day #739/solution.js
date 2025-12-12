// Quack (push/pop left, pull right) via three stacks.
// On underflow of one side, rebalance by moving half from the other side using a temp stack.
// Amortized O(1) per operation, O(1) extra memory beyond the three stacks.

class Quack {
  constructor() {
    this.front = []; // top = leftmost
    this.back = [];  // top = rightmost
    this.temp = [];
  }
  push(x) { this.front.push(x); }

  refillFront() {
    const n = this.back.length;
    const leftCount = Math.floor((n + 1) / 2), rightCount = n - leftCount;
    for (let i = 0; i < rightCount; i++) this.temp.push(this.back.pop());
    for (let i = 0; i < leftCount; i++) this.front.push(this.back.pop());
    while (this.temp.length) this.back.push(this.temp.pop());
  }
  refillBack() {
    const n = this.front.length;
    const rightCount = Math.floor((n + 1) / 2), leftCount = n - rightCount;
    for (let i = 0; i < leftCount; i++) this.temp.push(this.front.pop());
    for (let i = 0; i < rightCount; i++) this.back.push(this.front.pop());
    while (this.temp.length) this.front.push(this.temp.pop());
  }

  pop() {
    if (this.front.length === 0) this.refillFront();
    if (this.front.length === 0) throw new Error("empty");
    return this.front.pop();
  }
  pull() {
    if (this.back.length === 0) this.refillBack();
    if (this.back.length === 0) throw new Error("empty");
    return this.back.pop();
  }
}

const q = new Quack();
q.push(1); q.push(2); q.push(3);
console.log("pop:", q.pop());   // 3
console.log("pull:", q.pull()); // 1
q.push(4);
console.log("pull:", q.pull()); // 2
console.log("pop:", q.pop());   // 4
