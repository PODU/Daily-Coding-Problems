// Space-efficient bit array: pack bits into a Uint8Array (8 bits/byte). set/get O(1), space O(size/8).

class BitArray {
  init(size) {
    this.n = size;
    this.data = new Uint8Array((size + 7) >> 3);
  }
  set(i, val) {
    if (val) this.data[i >> 3] |= 1 << (i & 7);
    else this.data[i >> 3] &= ~(1 << (i & 7));
  }
  get(i) {
    return (this.data[i >> 3] >> (i & 7)) & 1;
  }
}

const b = new BitArray();
b.init(10);
b.set(2, 1); b.set(7, 1); b.set(7, 0); b.set(9, 1);
console.log(`${b.get(2)}${b.get(7)}${b.get(9)}${b.get(0)}`); // 1010
