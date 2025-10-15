// Day 436: Three stacks backed by one list using node-based singly-linked
// indices + a free list. push/pop are O(1) time, O(n) space overall.

class ThreeStacks {
  constructor() {
    this.list = [];          // single backing list of {val, prev}
    this.tops = [-1, -1, -1];
    this.free = [];
  }
  push(item, stackNumber) {
    let idx;
    if (this.free.length) { idx = this.free.pop(); this.list[idx] = { val: item, prev: this.tops[stackNumber] }; }
    else { idx = this.list.length; this.list.push({ val: item, prev: this.tops[stackNumber] }); }
    this.tops[stackNumber] = idx;
  }
  pop(stackNumber) {
    const idx = this.tops[stackNumber];
    if (idx === -1) throw new Error(`stack ${stackNumber} is empty`);
    const node = this.list[idx];
    this.tops[stackNumber] = node.prev;
    this.free.push(idx);
    return node.val;
  }
}

const st = new ThreeStacks();
st.push(1, 0); st.push(2, 0);
st.push(10, 1);
st.push(100, 2); st.push(200, 2);
console.log(st.pop(0)); // 2
console.log(st.pop(0)); // 1
console.log(st.pop(1)); // 10
console.log(st.pop(2)); // 200
console.log(st.pop(2)); // 100
