// XOR linked list simulated with a "memory" array; addresses are indices (0 = null).
// each node stores both = prevAddr XOR nextAddr. add: O(1), get(i): O(i). Space: O(n).
class Node {
  constructor(val) {
    this.val = val;
    this.both = 0;
  }
}

class XorList {
  constructor() {
    this.mem = [null]; // index 0 reserved as null
    this.head = 0;
    this.tail = 0;
  }
  alloc(n) {
    this.mem.push(n);
    return this.mem.length - 1;
  }
  add(val) {
    const addr = this.alloc(new Node(val));
    if (this.head === 0) {
      this.head = this.tail = addr;
      return;
    }
    this.mem[this.tail].both ^= addr;
    this.mem[addr].both = this.tail;
    this.tail = addr;
  }
  get(index) {
    let prev = 0;
    let cur = this.head;
    for (let i = 0; i < index && cur !== 0; i++) {
      const next = this.mem[cur].both ^ prev;
      prev = cur;
      cur = next;
    }
    return cur === 0 ? null : this.mem[cur];
  }
}

const l = new XorList();
for (const v of [10, 20, 30, 40]) l.add(v);
console.log([0, 1, 2, 3].map((i) => l.get(i).val).join(" "));
