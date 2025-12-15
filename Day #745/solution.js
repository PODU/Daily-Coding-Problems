// Bloom filter: fixed bit array, k hash functions via double hashing.
// check() may report false positives but never false negatives.
// Time: O(k) per add/check, Space: O(m bits). BigInt keeps 64-bit hashing exact.

class BloomFilter {
  constructor(size = 1000, numHashes = 4) {
    this.m = size;
    this.k = numHashes;
    this.bits = new Array(size).fill(false);
  }
  _hashes(s) {
    const MASK = (1n << 64n) - 1n;
    let h1 = 1469598103934665603n; // FNV-1a
    for (const ch of s) h1 = ((h1 ^ BigInt(ch.charCodeAt(0))) * 1099511628211n) & MASK;
    let h2 = 5381n; // djb2
    for (const ch of s) h2 = (((h2 << 5n) + h2) + BigInt(ch.charCodeAt(0))) & MASK;
    return [h1, h2];
  }
  add(s) {
    const [h1, h2] = this._hashes(s);
    const m = BigInt(this.m);
    for (let i = 0n; i < BigInt(this.k); i++) this.bits[Number((h1 + i * h2) % m)] = true;
  }
  check(s) {
    const [h1, h2] = this._hashes(s);
    const m = BigInt(this.m);
    for (let i = 0n; i < BigInt(this.k); i++) if (!this.bits[Number((h1 + i * h2) % m)]) return false;
    return true;
  }
}

const bf = new BloomFilter();
bf.add("apple");
bf.add("banana");
console.log("apple:", bf.check("apple"));   // true
console.log("banana:", bf.check("banana")); // true
console.log("cherry:", bf.check("cherry")); // false
