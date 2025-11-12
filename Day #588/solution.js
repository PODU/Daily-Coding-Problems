// Day 588: Space-efficient SparseArray backed by a Map of non-zero entries.
// Approach: store only non-zero indices. Time O(1) avg per op, Space O(#nonzero).
class SparseArray {
  constructor() {
    this.data = new Map();
    this.size = 0;
  }

  init(arr, size) {
    this.size = size;
    this.data = new Map();
    for (let i = 0; i < arr.length && i < size; i++) {
      if (arr[i] !== 0) this.data.set(i, arr[i]);
    }
  }

  set(i, val) {
    if (i < 0 || i >= this.size) throw new RangeError("index out of range");
    if (val === 0) this.data.delete(i);
    else this.data.set(i, val);
  }

  get(i) {
    if (i < 0 || i >= this.size) throw new RangeError("index out of range");
    return this.data.has(i) ? this.data.get(i) : 0;
  }
}

const sa = new SparseArray();
sa.init([0, 0, 0, 5, 0, 0, 9, 0], 8);
console.log("get(3) =", sa.get(3)); // 5
console.log("get(6) =", sa.get(6)); // 9
console.log("get(0) =", sa.get(0)); // 0
sa.set(1, 42);
console.log("after set(1,42), get(1) =", sa.get(1)); // 42
sa.set(3, 0);
console.log("after set(3,0), get(3) =", sa.get(3)); // 0
