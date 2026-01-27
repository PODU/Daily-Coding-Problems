// Day 970: Space-efficient SparseArray storing only non-zero entries.
// Approach: Map of index->value, default 0. Time O(1) avg per op, Space O(#nonzero).

class SparseArray {
  init(arr, size) {
    this.size = size;
    this.m = new Map();
    for (let i = 0; i < size; i++) if (arr[i] !== 0) this.m.set(i, arr[i]);
  }
  set(i, val) {
    if (i < 0 || i >= this.size) throw new RangeError('index');
    if (val === 0) this.m.delete(i);
    else this.m.set(i, val);
  }
  get(i) {
    if (i < 0 || i >= this.size) throw new RangeError('index');
    return this.m.has(i) ? this.m.get(i) : 0;
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
