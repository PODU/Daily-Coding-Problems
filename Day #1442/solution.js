// Day 1442: Implement a stack using only a max-heap.
// Approach: tag each item with an increasing counter; the heap's max key is
// always the most recently pushed item. push/pop O(log n).
class MaxHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(x) {
    const a = this.a; a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p][0] >= a[i][0]) break;
      [a[p], a[i]] = [a[i], a[p]]; i = p;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0;
      for (;;) {
        const l = 2 * i + 1, r = 2 * i + 2; let m = i;
        if (l < a.length && a[l][0] > a[m][0]) m = l;
        if (r < a.length && a[r][0] > a[m][0]) m = r;
        if (m === i) break;
        [a[m], a[i]] = [a[i], a[m]]; i = m;
      }
    }
    return top;
  }
}

class Stack {
  constructor() { this.heap = new MaxHeap(); this.counter = 0; }
  push(item) { this.heap.push([this.counter++, item]); }
  pop() {
    if (this.heap.size() === 0) throw new Error("pop from empty stack");
    return this.heap.pop()[1];
  }
}

const s = new Stack();
s.push(1); s.push(2); s.push(3);
console.log(s.pop(), s.pop(), s.pop()); // 3 2 1
