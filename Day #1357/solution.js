// XOR linked list simulated with array-backed memory & integer addresses (no real pointers).
// both = prevIdx ^ nextIdx, NULL sentinel = 0. add=O(1), get(i)=O(i) time, O(n) space.

const NULL = 0;

class XorList {
  constructor() {
    this.value = [0]; // index 0 reserved as NULL
    this.both = [0];
    this.head = NULL;
    this.tail = NULL;
  }
  add(v) {
    const idx = this.value.length;
    this.value.push(v);
    this.both.push(0);
    if (this.head === NULL) { this.head = this.tail = idx; return; }
    this.both[idx] = this.tail ^ NULL;      // prev=tail, next=NULL
    this.both[this.tail] ^= idx;            // append as next of old tail
    this.tail = idx;
  }
  get(index) {
    let prev = NULL, cur = this.head;
    for (let i = 0; i < index; i++) {
      const next = this.both[cur] ^ prev;
      prev = cur;
      cur = next;
    }
    return this.value[cur];
  }
}

const list = new XorList();
[10, 20, 30, 40].forEach(x => list.add(x));
for (let i = 0; i < 4; i++) console.log(list.get(i));
