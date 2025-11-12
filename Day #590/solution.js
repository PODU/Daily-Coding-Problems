// XOR doubly linked list simulated with a memory array; address = index.
// memory[0] is the null sentinel; both = prevIndex XOR nextIndex.
// add: O(1), get(i): O(i). Space O(n).

class Node {
  constructor(value) {
    this.value = value;
    this.both = 0; // prevIndex XOR nextIndex
  }
}

class XorList {
  constructor() {
    this.memory = [null]; // index 0 = null sentinel
    this.head = 0;
    this.tail = 0;
  }

  add(element) {
    const node = new Node(element);
    this.memory.push(node);
    const idx = this.memory.length - 1;
    if (this.head === 0) {
      this.head = this.tail = idx;
      return;
    }
    node.both = this.tail ^ 0;
    const tailNode = this.memory[this.tail];
    tailNode.both = (tailNode.both ^ 0) ^ idx;
    this.tail = idx;
  }

  get(index) {
    let prev = 0;
    let cur = this.head;
    for (let i = 0; i < index; i++) {
      const next = prev ^ this.memory[cur].both;
      prev = cur;
      cur = next;
    }
    return this.memory[cur].value;
  }
}

const list = new XorList();
list.add(10);
list.add(20);
list.add(30);
list.add(40);
console.log(list.get(0));
console.log(list.get(3));
