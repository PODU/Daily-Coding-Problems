// Three stacks in one list via interleaved indexing: logical pos p of stack s -> physical p*3+s.
// push/pop O(1) amortized. Space O(total elements). Single backing list.

class Stack {
  constructor() {
    this.list = [];          // single backing list
    this.sizes = [0, 0, 0];  // logical height of each stack
  }
  push(item, stackNumber) {
    const phys = this.sizes[stackNumber] * 3 + stackNumber;
    while (this.list.length <= phys) this.list.push(0);
    this.list[phys] = item;
    this.sizes[stackNumber]++;
  }
  pop(stackNumber) {
    if (this.sizes[stackNumber] === 0) throw new Error("empty stack");
    this.sizes[stackNumber]--;
    const phys = this.sizes[stackNumber] * 3 + stackNumber;
    return this.list[phys];
  }
}

const s = new Stack();
s.push(1, 0); s.push(2, 0);
s.push(10, 1);
s.push(100, 2); s.push(200, 2);
console.log(s.pop(0), s.pop(2), s.pop(1), s.pop(2), s.pop(0));
// 2 200 10 100 1
