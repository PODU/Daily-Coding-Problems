// O(1) LFU cache. Maps key->value/freq; each freq holds a Map of keys (insertion order
// = recency) so eviction picks least-frequent, then least-recent. Track minFreq.
// Time: O(1) per get/set, Space: O(capacity).

class LFUCache {
  constructor(capacity) {
    this.cap = capacity;
    this.val = new Map();      // key -> value
    this.freq = new Map();     // key -> frequency
    this.buckets = new Map();  // freq -> Map(key -> true), ordered by recency
    this.minFreq = 0;
  }

  _bump(key) {
    const f = this.freq.get(key);
    const bucket = this.buckets.get(f);
    bucket.delete(key);
    if (bucket.size === 0) {
      this.buckets.delete(f);
      if (this.minFreq === f) this.minFreq++;
    }
    this.freq.set(key, f + 1);
    if (!this.buckets.has(f + 1)) this.buckets.set(f + 1, new Map());
    this.buckets.get(f + 1).set(key, true);
  }

  get(key) {
    if (!this.val.has(key)) return null;
    this._bump(key);
    return this.val.get(key);
  }

  set(key, value) {
    if (this.cap <= 0) return;
    if (this.val.has(key)) {
      this.val.set(key, value);
      this._bump(key);
      return;
    }
    if (this.val.size >= this.cap) {
      const bucket = this.buckets.get(this.minFreq);
      const evictKey = bucket.keys().next().value; // least recent in min freq
      bucket.delete(evictKey);
      this.val.delete(evictKey);
      this.freq.delete(evictKey);
    }
    this.val.set(key, value);
    this.freq.set(key, 1);
    if (!this.buckets.has(1)) this.buckets.set(1, new Map());
    this.buckets.get(1).set(key, true);
    this.minFreq = 1;
  }
}

const c = new LFUCache(2);
c.set(1, 1);
c.set(2, 2);
console.log(c.get(1)); // 1
c.set(3, 3);           // evicts key 2
console.log(c.get(2)); // null
console.log(c.get(3)); // 3
c.set(4, 4);           // evicts key 1
console.log(c.get(1)); // null
console.log(c.get(3)); // 3
console.log(c.get(4)); // 4
