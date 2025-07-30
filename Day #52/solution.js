// Day 52: LRU cache using a JS Map (preserves insertion order). O(1) get/set.
// Time: O(1) per op, Space: O(n).
class LRUCache {
  constructor(n) {
    this.cap = n;
    this.map = new Map();
  }

  get(key) {
    if (!this.map.has(key)) return null;
    const value = this.map.get(key);
    this.map.delete(key);
    this.map.set(key, value); // move to most-recently used
    return value;
  }

  set(key, value) {
    if (this.map.has(key)) this.map.delete(key);
    this.map.set(key, value);
    if (this.map.size > this.cap) {
      this.map.delete(this.map.keys().next().value); // evict LRU
    }
  }
}

const c = new LRUCache(2);
c.set(1, 1);
c.set(2, 2);
console.log(c.get(1)); // 1
c.set(3, 3);           // evicts key 2
console.log(c.get(2)); // null
c.set(4, 4);           // evicts key 1
console.log(c.get(1)); // null
console.log(c.get(3)); // 3
console.log(c.get(4)); // 4
