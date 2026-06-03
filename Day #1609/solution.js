// Subscribers-per-hour over 24 hours via Fenwick/BIT. update(hour,val)+=, query(start,end)=inclusive range sum.
// Time O(log n) per op, Space O(n).
class Fenwick {
  constructor(n) {
    this.n = n;
    this.tree = new Array(n + 1).fill(0);
  }
  update(i, v) {
    for (i++; i <= this.n; i += i & -i) this.tree[i] += v;
  }
  pref(i) {
    let s = 0;
    for (i++; i > 0; i -= i & -i) s += this.tree[i];
    return s;
  }
  query(l, r) {
    return this.pref(r) - (l > 0 ? this.pref(l - 1) : 0);
  }
}

const bit = new Fenwick(24); // all zeros
bit.update(0, 5);
bit.update(3, 10);
bit.update(23, 2);
console.log(bit.query(0, 23)); // 17
console.log(bit.query(0, 3));  // 15
console.log(bit.query(1, 2));  // 0
