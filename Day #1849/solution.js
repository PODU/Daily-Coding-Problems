// Day 1849: SparseArray storing only non-zero entries in a Map.
// init O(N) once, set/get O(1) average. Space O(non-zero count).

class SparseArray {
  constructor(arr, size) {
    this.size = size;
    this.data = new Map();
    arr.forEach((v, i) => {
      if (v !== 0) this.data.set(i, v);
    });
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

function main() {
  const sa = new SparseArray([0, 0, 5, 0, 0, 0, 9, 0], 8);
  console.log(sa.get(2)); // 5
  console.log(sa.get(3)); // 0
  sa.set(3, 7);
  console.log(sa.get(3)); // 7
  sa.set(2, 0);
  console.log(sa.get(2)); // 0
}

main();
