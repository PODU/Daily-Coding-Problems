// Day 1791: All O(1) data structure (plus / minus / get_max / get_min).
//
// Idea: keep a doubly linked list of "buckets", one per distinct count,
// sorted by count. Each bucket owns the set of keys sitting at that count,
// and a Map points every key at its current bucket. Bumping a key up or
// down just moves it to a neighbouring bucket, so each operation touches a
// constant number of nodes. Min and max live right next to the sentinels.

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
    // Sentinels at both ends mean we never special-case front/back inserts.
    this.head = new Bucket(-Infinity);
    this.tail = new Bucket(Infinity);
    this.head.next = this.tail;
    this.tail.prev = this.head;
    this.keyBucket = new Map(); // key -> bucket it currently lives in
  }

  // Splice a fresh bucket for `count` right after `node`.
  _insertAfter(node, count) {
    const b = new Bucket(count);
    b.prev = node;
    b.next = node.next;
    node.next.prev = b;
    node.next = b;
    return b;
  }

  // Unlink an empty bucket from the list.
  _remove(b) {
    b.prev.next = b.next;
    b.next.prev = b.prev;
  }

  plus(key) {
    if (this.keyBucket.has(key)) {
      // Existing key: shift it one bucket to the right (count + 1).
      const cur = this.keyBucket.get(key);
      let nxt = cur.next;
      if (nxt === this.tail || nxt.count !== cur.count + 1) {
        // No bucket for count+1 yet, so make one next door.
        nxt = this._insertAfter(cur, cur.count + 1);
      }
      nxt.keys.add(key);
      this.keyBucket.set(key, nxt);
      cur.keys.delete(key);
      if (cur.keys.size === 0) {
        this._remove(cur); // the bucket we left is empty now
      }
    } else {
      // New key: it belongs in the count-1 bucket at the front.
      let first = this.head.next;
      if (first === this.tail || first.count !== 1) {
        first = this._insertAfter(this.head, 1);
      }
      first.keys.add(key);
      this.keyBucket.set(key, first);
    }
  }

  minus(key) {
    if (!this.keyBucket.has(key)) return; // unknown key -> no-op

    const cur = this.keyBucket.get(key);
    cur.keys.delete(key);

    if (cur.count === 1) {
      // Count would hit zero, so the key disappears entirely.
      this.keyBucket.delete(key);
    } else {
      // Shift the key one bucket to the left (count - 1).
      let prv = cur.prev;
      if (prv === this.head || prv.count !== cur.count - 1) {
        prv = this._insertAfter(cur.prev, cur.count - 1);
      }
      prv.keys.add(key);
      this.keyBucket.set(key, prv);
    }

    if (cur.keys.size === 0) {
      this._remove(cur);
    }
  }

  // Any key from the end buckets is a valid answer; picking the
  // lexicographically smallest keeps the output deterministic.
  get_max() {
    if (this.tail.prev === this.head) return "";
    return [...this.tail.prev.keys].reduce((a, b) => (a < b ? a : b));
  }

  get_min() {
    if (this.head.next === this.tail) return "";
    return [...this.head.next.keys].reduce((a, b) => (a < b ? a : b));
  }
}

const a = new AllOne();
a.plus("apple");
a.plus("apple");
a.plus("banana");
console.log(`max=${a.get_max()} min=${a.get_min()}`); // apple / banana

a.plus("banana");
a.plus("banana");
console.log(`max=${a.get_max()} min=${a.get_min()}`); // banana / apple

a.minus("apple");
a.minus("apple");
console.log(`max=${a.get_max()} min=${a.get_min()}`); // banana / banana
