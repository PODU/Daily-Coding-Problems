// All O(1) data structure (LeetCode 432).
// Doubly linked list of count-buckets (each holds a set of keys) + Map key->bucket. All ops O(1); space O(N).

class Bucket {
  constructor(count) {
    this.count = count;
    this.keys = new Set();
    this.prev = null;
    this.next = null;
  }
}

class AllOne {
  constructor() {
    this.head = new Bucket(-Infinity);
    this.tail = new Bucket(Infinity);
    this.head.next = this.tail;
    this.tail.prev = this.head;
    this.keyBucket = new Map();
  }
  _insertAfter(node, count) {
    const b = new Bucket(count);
    b.prev = node; b.next = node.next;
    node.next.prev = b; node.next = b;
    return b;
  }
  _remove(b) {
    b.prev.next = b.next; b.next.prev = b.prev;
  }
  plus(key) {
    if (this.keyBucket.has(key)) {
      const cur = this.keyBucket.get(key);
      let nxt = cur.next;
      if (nxt === this.tail || nxt.count !== cur.count + 1)
        nxt = this._insertAfter(cur, cur.count + 1);
      nxt.keys.add(key); this.keyBucket.set(key, nxt);
      cur.keys.delete(key);
      if (cur.keys.size === 0) this._remove(cur);
    } else {
      let first = this.head.next;
      if (first === this.tail || first.count !== 1)
        first = this._insertAfter(this.head, 1);
      first.keys.add(key); this.keyBucket.set(key, first);
    }
  }
  minus(key) {
    if (!this.keyBucket.has(key)) return;
    const cur = this.keyBucket.get(key);
    if (cur.count === 1) {
      this.keyBucket.delete(key);
    } else {
      let prv = cur.prev;
      if (prv === this.head || prv.count !== cur.count - 1)
        prv = this._insertAfter(cur.prev, cur.count - 1);
      prv.keys.add(key); this.keyBucket.set(key, prv);
    }
    cur.keys.delete(key);
    if (cur.keys.size === 0) this._remove(cur);
  }
  getMax() { return this.tail.prev === this.head ? "" : [...this.tail.prev.keys].reduce((a, b) => (a < b ? a : b)); }
  getMin() { return this.head.next === this.tail ? "" : [...this.head.next.keys].reduce((a, b) => (a < b ? a : b)); }
}

const a = new AllOne();
a.plus("a"); a.plus("a"); a.plus("a");
a.plus("b");
console.log("get_max:", a.getMax());
console.log("get_min:", a.getMin());
a.minus("a"); a.minus("a");
console.log("get_max:", a.getMax());
console.log("get_min:", a.getMin());
