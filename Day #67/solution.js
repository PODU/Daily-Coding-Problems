// LFU cache: key->(value,freq), freq->Map of keys (insertion-ordered = LRU within freq), track minFreq. O(1) get/set.
class LFUCache {
  constructor(capacity) {
    this.cap = capacity;
    this.minFreq = 0;
    this.values = new Map();   // key -> value
    this.freqs = new Map();    // key -> freq
    this.freqList = new Map(); // freq -> Map(key -> true), insertion order = LRU
  }

  _touch(key) {
    const f = this.freqs.get(key);
    const s = this.freqList.get(f);
    s.delete(key);
    if (s.size === 0) {
      this.freqList.delete(f);
      if (this.minFreq === f) this.minFreq = f + 1;
    }
    const nf = f + 1;
    this.freqs.set(key, nf);
    if (!this.freqList.has(nf)) this.freqList.set(nf, new Map());
    this.freqList.get(nf).set(key, true);
  }

  get(key) {
    if (!this.values.has(key)) return null;
    this._touch(key);
    return this.values.get(key);
  }

  set(key, value) {
    if (this.cap <= 0) return;
    if (this.values.has(key)) {
      this.values.set(key, value);
      this._touch(key);
      return;
    }
    if (this.values.size >= this.cap) {
      const s = this.freqList.get(this.minFreq);
      const evict = s.keys().next().value; // LRU at min freq
      s.delete(evict);
      if (s.size === 0) this.freqList.delete(this.minFreq);
      this.values.delete(evict);
      this.freqs.delete(evict);
    }
    this.values.set(key, value);
    this.freqs.set(key, 1);
    if (!this.freqList.has(1)) this.freqList.set(1, new Map());
    this.freqList.get(1).set(key, true);
    this.minFreq = 1;
  }
}

const show = (v) => console.log(v === null ? "null" : v);

const c = new LFUCache(2);
c.set(1, 1);
c.set(2, 2);
show(c.get(1)); // 1
c.set(3, 3);    // evicts key 2
show(c.get(2)); // null
show(c.get(3)); // 3
c.set(4, 4);    // evicts key 1
show(c.get(1)); // null
show(c.get(3)); // 3
show(c.get(4)); // 4
