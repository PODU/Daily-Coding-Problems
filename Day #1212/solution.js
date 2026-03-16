// Day 1212: Space-efficient bit array backed by a Uint32Array.
// Pack bits into 32-bit words; set/get use word index and bit offset. Time O(1) per op, Space O(size/32).
class BitArray {
  constructor(size) {
    this.n = size;
    this.words = new Uint32Array((size + 31) >> 5);
  }
  set(i, val) {
    if (val) this.words[i >> 5] |= (1 << (i & 31));
    else this.words[i >> 5] &= ~(1 << (i & 31));
  }
  get(i) {
    return (this.words[i >> 5] >>> (i & 31)) & 1;
  }
}

const b = new BitArray(10);
b.set(2, 1); b.set(7, 1); b.set(2, 0);
console.log(b.get(2), b.get(7), b.get(0)); // 0 1 0
