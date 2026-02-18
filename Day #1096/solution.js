// Day 1096: LFU Cache with O(1) get/set.
// Approach: Maps key->val/freq + freq->(insertion-ordered Map as LRU) + minFreq.
// Time: O(1) per op. Space: O(n).
class LFUCache {
  constructor(n) {
    this.cap = n;
    this.minFreq = 0;
    this.vals = new Map();
    this.freqs = new Map();
    this.lists = new Map(); // freq -> Map(key->true), insertion order = LRU
  }

  _touch(key) {
    const f = this.freqs.get(key);
    const fl = this.lists.get(f);
    fl.delete(key);
    if (fl.size === 0) {
      this.lists.delete(f);
      if (this.minFreq === f) this.minFreq++;
    }
    const nf = f + 1;
    this.freqs.set(key, nf);
    if (!this.lists.has(nf)) this.lists.set(nf, new Map());
    this.lists.get(nf).set(key, true);
  }

  get(key) {
    if (!this.vals.has(key)) return null;
    this._touch(key);
    return this.vals.get(key);
  }

  set(key, value) {
    if (this.cap <= 0) return;
    if (this.vals.has(key)) { this.vals.set(key, value); this._touch(key); return; }
    if (this.vals.size >= this.cap) {
      const fl = this.lists.get(this.minFreq);
      const evict = fl.keys().next().value; // LRU at min freq
      fl.delete(evict);
      if (fl.size === 0) this.lists.delete(this.minFreq);
      this.vals.delete(evict);
      this.freqs.delete(evict);
    }
    this.vals.set(key, value);
    this.freqs.set(key, 1);
    if (!this.lists.has(1)) this.lists.set(1, new Map());
    this.lists.get(1).set(key, true);
    this.minFreq = 1;
  }
}

const c = new LFUCache(2);
c.set(1, 1);
c.set(2, 2);
console.log(c.get(1));  // 1
c.set(3, 3);            // evicts key 2
console.log(c.get(2));  // null
console.log(c.get(3));  // 3
