// Three stacks in one array via fixed equal regions, each with its own top. O(1) push/pop.
class ThreeStacks {
  constructor(perStack = 3) {
    this.cap = perStack;
    this.arr = new Array(3 * perStack).fill(0);
    this.top = [0, 0, 0];
  }
  push(item, sn) {
    if (this.top[sn] >= this.cap) throw new Error("stack full");
    this.arr[sn * this.cap + this.top[sn]++] = item;
  }
  pop(sn) {
    if (this.top[sn] <= 0) throw new Error("stack empty");
    return this.arr[sn * this.cap + (--this.top[sn])];
  }
}

const s = new ThreeStacks(3);
s.push(1, 0); s.push(2, 0);
s.push(10, 1); s.push(20, 1);
s.push(100, 2);
console.log("stack0 pop: " + s.pop(0));
console.log("stack1 pop: " + s.pop(1));
console.log("stack2 pop: " + s.pop(2));
console.log("stack0 pop: " + s.pop(0));
