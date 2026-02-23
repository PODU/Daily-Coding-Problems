// Day 1117 - Three stacks backed by a single list
// Each entry stores [value, prev_index]; per-stack heads + free list give O(1)
// push/pop sharing one list. Space O(n).

class Stack3 {
  constructor() {
    this.list = [];          // entries: [value, prev]
    this.heads = [-1, -1, -1];
    this.free = [];
  }
  push(item, s) {
    let idx;
    if (this.free.length) {
      idx = this.free.pop();
      this.list[idx] = [item, this.heads[s]];
    } else {
      idx = this.list.length;
      this.list.push([item, this.heads[s]]);
    }
    this.heads[s] = idx;
  }
  pop(s) {
    const idx = this.heads[s];
    if (idx === -1) throw new Error("pop from empty stack " + s);
    const [value, prev] = this.list[idx];
    this.heads[s] = prev;
    this.free.push(idx);
    return value;
  }
}

const s = new Stack3();
s.push(1, 0); s.push(2, 0);
s.push(3, 1);
s.push(4, 2); s.push(5, 2);
console.log("pop(0):", s.pop(0)); // 2
console.log("pop(0):", s.pop(0)); // 1
console.log("pop(2):", s.pop(2)); // 5
console.log("pop(1):", s.pop(1)); // 3
console.log("pop(2):", s.pop(2)); // 4
