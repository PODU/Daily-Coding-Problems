// Day 1430: Space-efficient SparseArray for a mostly-zero array.
// Approach: store only non-zero indices in a Map. Time: O(1) avg per op, Space: O(#nonzero).

class SparseArray {
  constructor() {
    this.data = new Map();
    this.n = 0;
  }

  init(arr, size) {
    this.n = size;
    this.data.clear();
    for (let i = 0; i < size; i++) {
      if (arr[i] !== 0) this.data.set(i, arr[i]);
    }
  }

  set(i, val) {
    if (i < 0 || i >= this.n) throw new RangeError("index out of bounds");
    if (val === 0) this.data.delete(i);
    else this.data.set(i, val);
  }

  get(i) {
    if (i < 0 || i >= this.n) throw new RangeError("index out of bounds");
    return this.data.has(i) ? this.data.get(i) : 0;
  }
}

const sa = new SparseArray();
sa.init([0, 0, 5, 0, 0, 0, 9, 0], 8);
console.log(sa.get(2)); // 5
console.log(sa.get(3)); // 0
sa.set(3, 7);
console.log(sa.get(3)); // 7
sa.set(2, 0);
console.log(sa.get(2)); // 0
