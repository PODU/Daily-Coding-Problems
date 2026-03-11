// Max stack with O(1) push/pop/max.
// Auxiliary stack stores running maxima alongside main stack. All ops O(1); space O(N).

class MaxStack {
  constructor() {
    this.data = [];
    this.maxs = [];
  }
  push(val) {
    this.data.push(val);
    this.maxs.push(this.maxs.length === 0 ? val : Math.max(val, this.maxs[this.maxs.length - 1]));
  }
  pop() {
    // returns top, or null if empty
    if (this.data.length === 0) return null;
    this.maxs.pop();
    return this.data.pop();
  }
  max() {
    return this.maxs.length === 0 ? null : this.maxs[this.maxs.length - 1];
  }
}

const show = (v) => (v === null ? "null" : String(v));

const s = new MaxStack();
[3, 1, 5, 2].forEach((x) => s.push(x));
console.log("max:", show(s.max()));
console.log("pop:", show(s.pop()));
console.log("pop:", show(s.pop()));
console.log("max:", show(s.max()));
