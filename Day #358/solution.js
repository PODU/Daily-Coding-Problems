// All O(1) structure: doubly-linked list of count-buckets (increasing), each holds an ordered key set; map key->bucket.
// plus/minus move key to adjacent bucket. get_max=last bucket, get_min=first bucket. All O(1).

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
    this.head = new Bucket(-Infinity); // smallest
    this.tail = new Bucket(Infinity);  // largest
    this.head.next = this.tail;
    this.tail.prev = this.head;
    this.keyBucket = new Map();
  }
  _insertAfter(b, count) {
    const nb = new Bucket(count);
    nb.prev = b; nb.next = b.next;
    b.next.prev = nb; b.next = nb;
    return nb;
  }
  _remove(b) { b.prev.next = b.next; b.next.prev = b.prev; }

  plus(key) {
    if (!this.keyBucket.has(key)) {
      let first = this.head.next;
      if (first === this.tail || first.count !== 1) first = this._insertAfter(this.head, 1);
      first.keys.add(key);
      this.keyBucket.set(key, first);
      return;
    }
    const cur = this.keyBucket.get(key);
    let nxt = cur.next;
    if (nxt === this.tail || nxt.count !== cur.count + 1) nxt = this._insertAfter(cur, cur.count + 1);
    nxt.keys.add(key);
    this.keyBucket.set(key, nxt);
    cur.keys.delete(key);
    if (cur.keys.size === 0) this._remove(cur);
  }

  minus(key) {
    if (!this.keyBucket.has(key)) return;
    const cur = this.keyBucket.get(key);
    if (cur.count === 1) {
      cur.keys.delete(key);
      this.keyBucket.delete(key);
      if (cur.keys.size === 0) this._remove(cur);
      return;
    }
    let prv = cur.prev;
    if (prv === this.head || prv.count !== cur.count - 1) prv = this._insertAfter(cur.prev, cur.count - 1);
    prv.keys.add(key);
    this.keyBucket.set(key, prv);
    cur.keys.delete(key);
    if (cur.keys.size === 0) this._remove(cur);
  }

  getMax() { return this.tail.prev === this.head ? "" : [...this.tail.prev.keys].reduce((a, b) => (a < b ? a : b)); }
  getMin() { return this.head.next === this.tail ? "" : [...this.head.next.keys].reduce((a, b) => (a < b ? a : b)); }
}

const a = new AllOne();
a.plus("a"); a.plus("b"); a.plus("b");
a.plus("c"); a.plus("c"); a.plus("c");
console.log("max=" + a.getMax());
console.log("min=" + a.getMin());
a.minus("c"); a.minus("c");
console.log("max=" + a.getMax());
