// O(1) LFU cache: key->node map, freq->Map(ordered by recency), minFreq pointer.
// get/set O(1); Space O(capacity). Evict least-freq, tie -> least-recently-used.
class LFU {
  constructor(cap) {
    this.cap = cap;
    this.minFreq = 0;
    this.vals = new Map();
    this.freqs = new Map();
    this.lists = new Map(); // freq -> Map(key->true), insertion order: first=LRU
  }
  _touch(key) {
    const f = this.freqs.get(key);
    this.lists.get(f).delete(key);
    if (this.lists.get(f).size === 0) {
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
  set(key, val) {
    if (this.cap <= 0) return;
    if (this.vals.has(key)) { this.vals.set(key, val); this._touch(key); return; }
    if (this.vals.size >= this.cap) {
      const lru = this.lists.get(this.minFreq).keys().next().value;
      this.lists.get(this.minFreq).delete(lru);
      if (this.lists.get(this.minFreq).size === 0) this.lists.delete(this.minFreq);
      this.vals.delete(lru);
      this.freqs.delete(lru);
    }
    this.vals.set(key, val);
    this.freqs.set(key, 1);
    if (!this.lists.has(1)) this.lists.set(1, new Map());
    this.lists.get(1).set(key, true);
    this.minFreq = 1;
  }
}
function show(v) { console.log(v === null ? "null" : v); }
function main() {
  const c = new LFU(2);
  c.set(1, 1); c.set(2, 2);
  show(c.get(1));
  c.set(3, 3);
  show(c.get(2));
  show(c.get(3));
  c.set(4, 4);
  show(c.get(1));
  show(c.get(3));
  show(c.get(4));
}
main();
