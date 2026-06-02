// Stack via a single MAX-heap keyed by a monotonic counter; newest key is largest so pops first (LIFO).
// push/pop O(log n), space O(n). Binary max-heap implemented by counter key.
"use strict";

class MaxHeap {
  constructor() { this.a = []; } // entries: {key, val}
  _swap(i, j) { const t = this.a[i]; this.a[i] = this.a[j]; this.a[j] = t; }
  push(key, val) {
    this.a.push({ key, val });
    let i = this.a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.a[p].key >= this.a[i].key) break;
      this._swap(i, p); i = p;
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
      while (true) {
        const l = 2 * i + 1, r = 2 * i + 2;
        let m = i;
        if (l < this.a.length && this.a[l].key > this.a[m].key) m = l;
        if (r < this.a.length && this.a[r].key > this.a[m].key) m = r;
        if (m === i) break;
        this._swap(i, m); i = m;
      }
    }
    return top;
  }
  get size() { return this.a.length; }
}

class StackViaHeap {
  constructor() { this.heap = new MaxHeap(); this.counter = 0; }
  push(x) { this.heap.push(this.counter++, x); }
  pop() {
    if (this.heap.size === 0) throw new Error("pop from empty stack");
    return this.heap.pop().val;
  }
  get empty() { return this.heap.size === 0; }
}

function main() {
  const s = new StackViaHeap();
  s.push(1); s.push(2); s.push(3);
  const out = [];
  out.push(s.pop()); // 3
  out.push(s.pop()); // 2
  s.push(4);
  out.push(s.pop()); // 4
  out.push(s.pop()); // 1
  console.log(out.join(" "));
}

main();
