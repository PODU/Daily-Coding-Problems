// Day 438: Stack via a max-heap. Each push tags the item with an increasing
// counter; the heap's max counter is the most-recent item. push/pop O(log n).

class MaxHeap {
  constructor() { this.a = []; } // each: [counter, value]
  _swap(i, j) { [this.a[i], this.a[j]] = [this.a[j], this.a[i]]; }
  push(node) {
    this.a.push(node);
    let i = this.a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.a[p][0] >= this.a[i][0]) break;
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
        const l = 2 * i + 1, r = 2 * i + 2; let big = i;
        if (l < this.a.length && this.a[l][0] > this.a[big][0]) big = l;
        if (r < this.a.length && this.a[r][0] > this.a[big][0]) big = r;
        if (big === i) break;
        this._swap(i, big); i = big;
      }
    }
    return top;
  }
  get size() { return this.a.length; }
}

class StackFromHeap {
  constructor() { this.heap = new MaxHeap(); this.counter = 0; }
  push(item) { this.heap.push([this.counter++, item]); }
  pop() {
    if (this.heap.size === 0) throw new Error("stack is empty");
    return this.heap.pop()[1];
  }
}

const s = new StackFromHeap();
s.push(1); s.push(2); s.push(3);
console.log(s.pop()); // 3
console.log(s.pop()); // 2
s.push(4);
console.log(s.pop()); // 4
console.log(s.pop()); // 1
