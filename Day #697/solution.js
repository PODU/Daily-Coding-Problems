// Day 697: LRU cache with O(1) get/set.
// Approach: JS Map preserves insertion order -> delete+reinsert marks recency.
// get/set O(1) time, O(n) space.
class LRUCache {
  constructor(n) { this.cap = n; this.map = new Map(); }
  get(key) {
    if (!this.map.has(key)) return null;
    const v = this.map.get(key);
    this.map.delete(key);
    this.map.set(key, v);
    return v;
  }
  set(key, value) {
    if (this.map.has(key)) this.map.delete(key);
    this.map.set(key, value);
    if (this.map.size > this.cap) this.map.delete(this.map.keys().next().value);
  }
}

const c = new LRUCache(2);
c.set(1, 1); c.set(2, 2);
console.log(c.get(1)); // 1
c.set(3, 3);           // evicts key 2
console.log(c.get(2)); // null
console.log(c.get(3)); // 3
