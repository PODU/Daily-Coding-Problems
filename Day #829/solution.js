// Day 829: O(1) data structure with plus/minus/get_max/get_min.
// Doubly-linked list of count-buckets (each a Set of keys) + key->bucket map.
// All operations O(1) time; O(K) space for K distinct keys.
"use strict";

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
    b.prev = node;
    b.next = node.next;
    node.next.prev = b;
    node.next = b;
    return b;
  }

  _remove(node) {
    node.prev.next = node.next;
    node.next.prev = node.prev;
  }

  plus(key) {
    if (this.keyBucket.has(key)) {
      const cur = this.keyBucket.get(key);
      const newCount = cur.count + 1;
      let nxt = cur.next;
      if (nxt.count !== newCount) nxt = this._insertAfter(cur, newCount);
      nxt.keys.add(key);
      this.keyBucket.set(key, nxt);
      cur.keys.delete(key);
      if (cur.keys.size === 0) this._remove(cur);
    } else {
      let first = this.head.next;
      if (first.count !== 1) first = this._insertAfter(this.head, 1);
      first.keys.add(key);
      this.keyBucket.set(key, first);
    }
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
    const newCount = cur.count - 1;
    let prv = cur.prev;
    if (prv.count !== newCount) prv = this._insertAfter(cur.prev, newCount);
    prv.keys.add(key);
    this.keyBucket.set(key, prv);
    cur.keys.delete(key);
    if (cur.keys.size === 0) this._remove(cur);
  }

  get_max() {
    if (this.tail.prev === this.head) return "";
    return [...this.tail.prev.keys].reduce((a, b) => (a < b ? a : b));
  }

  get_min() {
    if (this.head.next === this.tail) return "";
    return [...this.head.next.keys].reduce((a, b) => (a < b ? a : b));
  }
}

function main() {
  const ao = new AllOne();
  ao.plus("a");
  ao.plus("b");
  ao.plus("b");
  console.log("get_max: " + ao.get_max()); // b
  console.log("get_min: " + ao.get_min()); // a
  ao.minus("b");
  ao.minus("b");
  console.log("get_max: " + ao.get_max()); // a
}

main();
