// LRU cache via JS Map (insertion-ordered): delete+set to refresh, first key is LRU. O(1) per get/set. Space O(capacity).
class LRUCache {
  constructor(n) {
    this.cap = n;
    this.map = new Map();
  }

  get(key) {
    if (!this.map.has(key)) return null;
    const val = this.map.get(key);
    this.map.delete(key);
    this.map.set(key, val);
    return val;
  }

  set(key, value) {
    if (this.map.has(key)) this.map.delete(key);
    this.map.set(key, value);
    if (this.map.size > this.cap) {
      const lru = this.map.keys().next().value;
      this.map.delete(lru);
    }
  }
}

function printGet(cache, key) {
  const v = cache.get(key);
  console.log(v === null ? "null" : v);
}

const c = new LRUCache(2);
c.set(1, 1);
c.set(2, 2);
printGet(c, 1);   // 1
c.set(3, 3);      // evicts 2
printGet(c, 2);   // null
c.set(4, 4);      // evicts 1
printGet(c, 1);   // null
printGet(c, 3);   // 3
printGet(c, 4);   // 4
