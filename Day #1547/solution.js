// XOR linked list simulated with arrays indexed by integer addresses.
// add appends in O(1); get traverses with nextAddr = curBoth XOR prevAddr in O(index). Space O(n).

class XorList {
  constructor() {
    this.val = [0];   // address 0 is sentinel/null
    this.both = [0];  // prevAddr XOR nextAddr
    this.head = 0;
    this.tail = 0;
  }
  add(v) {
    const addr = this.val.length;
    this.val.push(v);
    this.both.push(0);
    if (this.head === 0) {
      this.head = this.tail = addr;
    } else {
      this.both[this.tail] ^= addr;
      this.both[addr] ^= this.tail;
      this.tail = addr;
    }
  }
  get(index) {
    let prev = 0, cur = this.head;
    for (let i = 0; i < index; i++) {
      const next = this.both[cur] ^ prev;
      prev = cur;
      cur = next;
    }
    return this.val[cur];
  }
}

const list = new XorList();
for (const v of [10, 20, 30, 40, 50]) list.add(v);
console.log(list.get(0));
console.log(list.get(2));
console.log(list.get(4));
