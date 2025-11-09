// Day 574: Space-efficient bit array backed by a Uint32Array (32 bits/word).
// set/get are O(1); storage is ceil(size/32) words.
'use strict';

class BitArray {
  constructor(size) {
    this.n = size;
    this.words = new Uint32Array((size + 31) >>> 5);
  }
  set(i, val) {
    if (i < 0 || i >= this.n) throw new RangeError('index');
    if (val) this.words[i >>> 5] |= (1 << (i & 31));
    else this.words[i >>> 5] &= ~(1 << (i & 31));
  }
  get(i) {
    if (i < 0 || i >= this.n) throw new RangeError('index');
    return (this.words[i >>> 5] >>> (i & 31)) & 1;
  }
}

const b = new BitArray(8);
b.set(0, 1);
b.set(3, 1);
console.log('get(0) =', b.get(0)); // 1
console.log('get(1) =', b.get(1)); // 0
console.log('get(3) =', b.get(3)); // 1
b.set(3, 0);
console.log('get(3) =', b.get(3)); // 0
