// Max stack with O(1) push/pop/max using an auxiliary stack of running maxima.
// All operations O(1) time; O(n) space.
class MaxStack {
  constructor() {
    this.data = [];
    this.maxs = [];
  }
  push(val) {
    this.data.push(val);
    if (this.maxs.length === 0 || val >= this.maxs[this.maxs.length - 1]) {
      this.maxs.push(val);
    } else {
      this.maxs.push(this.maxs[this.maxs.length - 1]);
    }
  }
  pop() {
    if (this.data.length === 0) return null;
    this.maxs.pop();
    return this.data.pop();
  }
  max() {
    return this.maxs.length === 0 ? null : this.maxs[this.maxs.length - 1];
  }
}

function main() {
  const s = new MaxStack();
  for (const v of [1, 5, 3, 9, 2]) s.push(v);
  console.log("max: " + s.max());
  console.log("pop: " + s.pop());
  console.log("pop: " + s.pop());
  console.log("max: " + s.max());
}

main();
