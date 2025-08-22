// Day 154: Stack using only a max-heap. Tag each item with an increasing
// priority so the heap always pops the most recent. push/pop O(log n).
'use strict';

class MaxHeap {
  constructor() { this.a = []; }
  _swap(i, j) { [this.a[i], this.a[j]] = [this.a[j], this.a[i]]; }
  push(node) {
    this.a.push(node);
    let i = this.a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.a[p].prio >= this.a[i].prio) break;
      this._swap(i, p); i = p;
    }
  }
  pop() {
    const top = this.a[0];
    const last = this.a.pop();
    if (this.a.length) {
      this.a[0] = last;
      let i = 0;
      const n = this.a.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, big = i;
        if (l < n && this.a[l].prio > this.a[big].prio) big = l;
        if (r < n && this.a[r].prio > this.a[big].prio) big = r;
        if (big === i) break;
        this._swap(i, big); i = big;
      }
    }
    return top;
  }
  get size() { return this.a.length; }
}

class HeapStack {
  constructor() { this.heap = new MaxHeap(); this.counter = 0; }
  push(item) { this.heap.push({ prio: this.counter++, value: item }); }
  pop() {
    if (this.heap.size === 0) throw new Error('pop from empty stack');
    return this.heap.pop().value;
  }
  get empty() { return this.heap.size === 0; }
}

const s = new HeapStack();
s.push(1); s.push(2); s.push(3);
console.log(s.pop()); // 3
console.log(s.pop()); // 2
s.push(4);
console.log(s.pop()); // 4
console.log(s.pop()); // 1
