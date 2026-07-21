// Day 1853: Stack with O(1) push/pop/max.
// Keep an auxiliary stack of running maxima alongside the data stack. All ops O(1) time, O(n) space.

class MaxStack {
  constructor() {
    this.data = [];
    this.maxs = [];
  }
  push(v) {
    this.data.push(v);
    this.maxs.push(this.maxs.length === 0 ? v : Math.max(this.maxs[this.maxs.length - 1], v));
  }
  pop() {
    if (this.data.length === 0) throw new Error('empty stack');
    this.maxs.pop();
    return this.data.pop();
  }
  max() {
    if (this.maxs.length === 0) throw new Error('empty stack');
    return this.maxs[this.maxs.length - 1];
  }
}

const s = new MaxStack();
s.push(1); s.push(5); s.push(3);
console.log(s.max()); // 5
console.log(s.pop()); // 3
console.log(s.max()); // 5
console.log(s.pop()); // 5
console.log(s.max()); // 1
