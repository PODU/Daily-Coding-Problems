// XOR linked list via memory-pool simulation: nodes stored in an array, indices act as
// "addresses"; both = prevId XOR nextId (sentinel 0 = null, real nodes start at 1).
// add O(1) with tail tracking, get O(index). O(1) extra per node.
'use strict';

class Node {
  constructor(value) {
    this.value = value;
    this.both = 0; // prevId XOR nextId
  }
}

class XorList {
  constructor() {
    this.pool = [null]; // index 0 reserved as null sentinel
    this.head = 0;
    this.tail = 0;
  }

  add(element) {
    const node = new Node(element);
    this.pool.push(node);
    const id = this.pool.length - 1;
    if (this.head === 0) {
      this.head = this.tail = id;
    } else {
      this.pool[this.tail].both ^= id; // old tail next becomes id
      node.both = this.tail;           // prev = old tail, next = 0
      this.tail = id;
    }
  }

  get(index) {
    let prev = 0;
    let cur = this.head;
    for (let i = 0; i < index && cur !== 0; i++) {
      const next = this.pool[cur].both ^ prev;
      prev = cur;
      cur = next;
    }
    if (cur === 0) throw new RangeError('index out of range');
    return this.pool[cur].value;
  }
}

function main() {
  const list = new XorList();
  [10, 20, 30, 40, 50].forEach((v) => list.add(v));
  console.log(`get(0) = ${list.get(0)}`);
  console.log(`get(2) = ${list.get(2)}`);
  console.log(`get(4) = ${list.get(4)}`);
}

main();
