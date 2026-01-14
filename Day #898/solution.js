// Stack via max-heap: tag each push with increasing priority; heap pops highest priority (most recent). O(log n)/op, O(n) space.
'use strict';

class MaxHeap {
  constructor() { this.a = []; } // each: {p, v}
  _swap(i, j) { const t = this.a[i]; this.a[i] = this.a[j]; this.a[j] = t; }
  push(node) {
    this.a.push(node);
    let i = this.a.length - 1;
    while (i > 0) {
      const par = (i - 1) >> 1;
      if (this.a[par].p >= this.a[i].p) break;
      this._swap(par, i); i = par;
    }
  }
  pop() {
    const n = this.a.length;
    if (n === 0) return undefined;
    const top = this.a[0];
    const last = this.a.pop();
    if (n > 1) {
      this.a[0] = last;
      let i = 0;
      const m = this.a.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, big = i;
        if (l < m && this.a[l].p > this.a[big].p) big = l;
        if (r < m && this.a[r].p > this.a[big].p) big = r;
        if (big === i) break;
        this._swap(big, i); i = big;
      }
    }
    return top;
  }
  get size() { return this.a.length; }
}

class HeapStack {
  constructor() { this.h = new MaxHeap(); this.counter = 0; }
  push(v) { this.h.push({ p: this.counter++, v }); }
  pop() {
    if (this.h.size === 0) throw new Error("pop from empty stack");
    return this.h.pop().v;
  }
}

const s = new HeapStack();
s.push(1); s.push(2); s.push(3);
console.log(s.pop()); // 3
console.log(s.pop()); // 2
s.push(4);
console.log(s.pop()); // 4
console.log(s.pop()); // 1
