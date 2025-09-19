// Day 301: Bloom filter - fixed-size probabilistic set. No false negatives.
// add/check O(k); space O(m) bits.
class BloomFilter {
  constructor(m, k) { this.bits = new Array(m).fill(false); this.k = k; }
  _hash(s) {
    let h = 2166136261;
    for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619); }
    return h >>> 0;
  }
  _h(v, i) {
    const h1 = this._hash(v), h2 = this._hash(v + "#salt");
    return ((h1 + i * h2) >>> 0) % this.bits.length;
  }
  add(v) { for (let i = 0; i < this.k; i++) this.bits[this._h(v, i)] = true; }
  check(v) { for (let i = 0; i < this.k; i++) if (!this.bits[this._h(v, i)]) return false; return true; }
}
const bf = new BloomFilter(1000, 4);
bf.add("apple"); bf.add("banana");
console.log(bf.check("apple"));
console.log(bf.check("banana"));
console.log(bf.check("cherry"));
