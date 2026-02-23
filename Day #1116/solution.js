// Day 1116 - Quack: push/pop left, pull right, using three stacks.
// Two stacks L (top=leftmost) and R (top=rightmost); rebalance by splitting the
// other in half via scratch stack T. Amortized O(1) per op, O(1) extra memory.

class Quack {
  constructor() {
    this.L = [];
    this.R = [];
    this.T = [];
  }
  push(x) { this.L.push(x); }

  _rebalanceToL() {
    const m = this.R.length;
    const rightCount = Math.floor(m / 2);
    for (let i = 0; i < rightCount; i++) this.T.push(this.R.pop());
    while (this.R.length) this.L.push(this.R.pop());
    while (this.T.length) this.R.push(this.T.pop());
  }

  _rebalanceToR() {
    const m = this.L.length;
    const leftCount = Math.floor(m / 2);
    for (let i = 0; i < leftCount; i++) this.T.push(this.L.pop());
    while (this.L.length) this.R.push(this.L.pop());
    while (this.T.length) this.L.push(this.T.pop());
  }

  pop() {
    if (this.L.length === 0) this._rebalanceToL();
    return this.L.pop();
  }

  pull() {
    if (this.R.length === 0) this._rebalanceToR();
    return this.R.pop();
  }
}

const q = new Quack();
for (const x of [1, 2, 3]) q.push(x);
console.log("pop:", q.pop());   // 3
console.log("pull:", q.pull()); // 1
for (const x of [4, 5]) q.push(x);
console.log("pull:", q.pull()); // 2
console.log("pop:", q.pop());   // 5
console.log("pull:", q.pull()); // 4
