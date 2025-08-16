// Day 134: SparseArray storing only non-zero entries in a hash map.
// init O(n) once, set/get O(1) average. Space O(#nonzero).
class SparseArray {
  constructor() {
    this.data = new Map();
    this.size = 0;
  }
  init(arr, sz) {
    this.size = sz;
    this.data = new Map();
    for (let i = 0; i < arr.length && i < sz; i++)
      if (arr[i] !== 0) this.data.set(i, arr[i]);
  }
  set(i, val) {
    if (i < 0 || i >= this.size) throw new RangeError("index");
    if (val === 0) this.data.delete(i);
    else this.data.set(i, val);
  }
  get(i) {
    if (i < 0 || i >= this.size) throw new RangeError("index");
    return this.data.has(i) ? this.data.get(i) : 0;
  }
}

const sa = new SparseArray();
sa.init([0, 0, 7, 0, 0, 0, 3, 0], 8);
console.log(sa.get(2)); // 7
console.log(sa.get(0)); // 0
sa.set(0, 5);
console.log(sa.get(0)); // 5
sa.set(2, 0);
console.log(sa.get(2)); // 0
