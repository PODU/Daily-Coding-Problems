// Day 848: LRU cache with O(1) get/set using a JS Map (preserves insertion order).
// Re-insert on access to mark recent; first key is LRU. O(1) per op.
class LRUCache {
  constructor(n) {
    this.cap = n;
    this.map = new Map();
  }
  get(key) {
    if (!this.map.has(key)) return null;
    const v = this.map.get(key);
    this.map.delete(key);
    this.map.set(key, v);
    return v;
  }
  set(key, value) {
    if (this.map.has(key)) {
      this.map.delete(key);
    } else if (this.map.size === this.cap) {
      this.map.delete(this.map.keys().next().value);
    }
    this.map.set(key, value);
  }
}

const c = new LRUCache(2);
c.set(1, 1);
c.set(2, 2);
console.log(c.get(1)); // 1
c.set(3, 3);           // evicts 2
console.log(c.get(2)); // null
c.set(4, 4);           // evicts 1
console.log(c.get(1)); // null
console.log(c.get(3)); // 3
console.log(c.get(4)); // 4
