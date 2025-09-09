// Fenwick/Binary Indexed Tree over 24 hours.
// update: O(log n), query (prefix-diff): O(log n). Space O(n).
class BIT {
  constructor(n) {
    this.n = n;
    this.tree = new Array(n + 1).fill(0);
  }
  add(i, v) {                 // 0-based index
    for (i++; i <= this.n; i += i & -i) this.tree[i] += v;
  }
  prefix(i) {                 // sum of [0..i], 0-based
    let s = 0;
    for (i++; i > 0; i -= i & -i) s += this.tree[i];
    return s;
  }
  query(l, r) {               // inclusive [l..r]
    return this.prefix(r) - (l > 0 ? this.prefix(l - 1) : 0);
  }
  update(hour, value) {
    this.add(hour, value);
  }
}

const bit = new BIT(24);
bit.update(2, 5);
bit.update(5, 3);
bit.update(23, 10);
console.log("query(2,5) = " + bit.query(2, 5));
console.log("query(0,23) = " + bit.query(0, 23));
