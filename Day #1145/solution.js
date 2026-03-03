// Day 1145: Bloom filter - fixed-size bit array, k hashes.
// add/check O(k); check has false positives but never false negatives.
class BloomFilter {
  constructor(m, k) {
    this.m = m;
    this.k = k;
    this.bits = new Uint8Array(m);
  }
  _hash(v, i) {
    let h = (2166136261 ^ (i + 1)) >>> 0;
    for (let j = 0; j < v.length; j++) {
      h ^= v.charCodeAt(j);
      h = Math.imul(h, 16777619) >>> 0;
    }
    return h % this.m;
  }
  add(v) {
    for (let i = 0; i < this.k; i++) this.bits[this._hash(v, i)] = 1;
  }
  check(v) {
    for (let i = 0; i < this.k; i++) if (!this.bits[this._hash(v, i)]) return false;
    return true;
  }
}

const bf = new BloomFilter(1000, 4);
bf.add("apple");
bf.add("banana");
console.log(bf.check("apple"), bf.check("banana"), bf.check("cherry"));
// true true false (likely)
