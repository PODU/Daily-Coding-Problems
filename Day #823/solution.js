// Space-efficient bit array using 32-bit words; index>>5 picks word, 1<<(index&31) picks bit.
// Time: O(1) per op, Space: O(n/32 words).

class BitArray {
  constructor(size) {
    this.words = new Int32Array((size + 31) >> 5);
  }
  set(i, val) {
    if (val) this.words[i >> 5] |= (1 << (i & 31));
    else this.words[i >> 5] &= ~(1 << (i & 31));
  }
  get(i) {
    return (this.words[i >> 5] >> (i & 31)) & 1;
  }
}

const ba = new BitArray(16);
ba.set(0, 1);
ba.set(5, 1);
ba.set(15, 1);
console.log("get(0)=" + ba.get(0));
console.log("get(1)=" + ba.get(1));
console.log("get(5)=" + ba.get(5));
console.log("get(15)=" + ba.get(15));
