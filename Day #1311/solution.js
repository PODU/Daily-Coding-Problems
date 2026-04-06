// Quack (deque) via three stacks. On underflow of one end, rebalance by moving
// half the elements through the third stack -> O(1) amortized, O(1) extra memory.
'use strict';

class Quack {
  constructor() { this.L = []; this.R = []; this.T = []; }

  static rebalance(src, dst, tmp) {
    const n = src.length, k = n >> 1; // k elements stay in src
    for (let i = 0; i < k; i++) tmp.push(src.pop());
    for (let i = 0; i < n - k; i++) dst.push(src.pop());
    for (let i = 0; i < k; i++) src.push(tmp.pop());
  }
  push(x) { this.L.push(x); }
  pop()  { if (this.L.length === 0) Quack.rebalance(this.R, this.L, this.T); return this.L.pop(); }
  pull() { if (this.R.length === 0) Quack.rebalance(this.L, this.R, this.T); return this.R.pop(); }
}

const q = new Quack();
q.push(1); q.push(2); q.push(3);
console.log(q.pop());  // 3
console.log(q.pull()); // 1
q.push(4);
console.log(q.pull()); // 2
console.log(q.pop());  // 4
