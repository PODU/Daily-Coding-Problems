// Max stack: each entry stores the running max so far, giving O(1) push/pop/max.
// Time: O(1) per operation, Space: O(n).

class MaxStack {
  constructor() { this.st = []; } // entries: [value, maxSoFar]
  push(v) {
    const mx = this.st.length === 0 ? v : Math.max(v, this.st[this.st.length - 1][1]);
    this.st.push([v, mx]);
  }
  pop() {
    if (this.st.length === 0) return null;
    return this.st.pop()[0];
  }
  max() {
    if (this.st.length === 0) return null;
    return this.st[this.st.length - 1][1];
  }
}

const s = new MaxStack();
s.push(1); s.push(3); s.push(2);
console.log("max:", s.max()); // 3
console.log("pop:", s.pop()); // 2
console.log("max:", s.max()); // 3
console.log("pop:", s.pop()); // 3
console.log("max:", s.max()); // 1
