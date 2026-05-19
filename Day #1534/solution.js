// Space-efficient bit array packing 32 bits per word.
// init(size), set(i,val), get(i): each O(1); space O(size/32) words.
class BitArray {
  init(size) {
    this.n = size;
    this.words = new Int32Array((size + 31) >> 5);
  }
  set(i, val) {
    if (val) this.words[i >> 5] |= 1 << (i & 31);
    else this.words[i >> 5] &= ~(1 << (i & 31));
  }
  get(i) {
    return (this.words[i >> 5] >>> (i & 31)) & 1;
  }
}

const b = new BitArray();
b.init(10);
b.set(1, 1);
b.set(4, 1);
b.set(4, 0);
b.set(9, 1);
console.log(b.get(1), b.get(4), b.get(9), b.get(0));
// expected: 1 0 1 0
