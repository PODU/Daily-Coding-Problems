// Bloom filter: fixed bit array (1000 bits) + k=3 hashes via double hashing.
// add/check are O(k); space O(m) bits. check has false positives, no false negatives.
'use strict';

class BloomFilter {
  constructor() {
    this.SIZE = 1000;
    this.K = 3;
    this.bits = new Array(this.SIZE).fill(false);
  }

  _hash(s, seed) {
    let h = seed >>> 0;
    for (let i = 0; i < s.length; i++) {
      h = Math.imul(h ^ s.charCodeAt(i), 16777619) >>> 0;
    }
    return h >>> 0;
  }

  _baseHashes(v) {
    return [this._hash(v, 2166136261), this._hash(v, 0x811c9dc5 ^ 0x55)];
  }

  add(v) {
    const [h1, h2] = this._baseHashes(v);
    for (let i = 0; i < this.K; i++) this.bits[(h1 + i * h2) % this.SIZE] = true;
  }

  check(v) {
    const [h1, h2] = this._baseHashes(v);
    for (let i = 0; i < this.K; i++) if (!this.bits[(h1 + i * h2) % this.SIZE]) return false;
    return true;
  }
}

function main() {
  const bf = new BloomFilter();
  const added = ['apple', 'banana', 'cherry'];
  added.forEach((s) => bf.add(s));

  console.log('Added values (expect all true):');
  added.forEach((s) => console.log(`  check(${s}) = ${bf.check(s)}`));

  console.log('Non-added values (expect mostly false):');
  ['date', 'elderberry', 'fig', 'grape'].forEach((s) =>
    console.log(`  check(${s}) = ${bf.check(s)}`)
  );
}

main();
