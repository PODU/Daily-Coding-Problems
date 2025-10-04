// Day 365: "Quack" deque (push/pop left, pull right) from three stacks.
// l holds the left part (left end on top), r the right part (right end on top);
// tmp is a transient helper used only to re-split when one side empties.
// Rebalance moves k after k ops -> O(1) amortized, O(1) extra memory.
'use strict';

class Quack {
  constructor() {
    this.l = [];
    this.r = [];
    this.tmp = [];
  }

  push(x) {
    this.l.push(x);
  }

  rebalance(to, from, toCount) {
    const n = from.length;
    for (let i = 0; i < n - toCount; i++) this.tmp.push(from.pop());
    for (let i = 0; i < toCount; i++) to.push(from.pop());
    while (this.tmp.length) from.push(this.tmp.pop());
  }

  pop() {
    if (this.l.length === 0) this.rebalance(this.l, this.r, (this.r.length + 1) >> 1);
    return this.l.pop();
  }

  pull() {
    if (this.r.length === 0) this.rebalance(this.r, this.l, (this.l.length + 1) >> 1);
    return this.r.pop();
  }
}

const q = new Quack();
q.push(1); q.push(2); q.push(3);
console.log(q.pop());   // 3
console.log(q.pull());  // 1
q.push(4); q.push(5);
console.log(q.pull());  // 2
console.log(q.pop());   // 5
console.log(q.pop());   // 4
