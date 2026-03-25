// Day 1267: Range sum query with preprocessing.
// Prefix-sum array: O(n) preprocess, O(1) per sum(i, j) query.
class RangeSum {
  constructor(L) {
    this.prefix = [0];
    for (let i = 0; i < L.length; i++) this.prefix.push(this.prefix[i] + L[i]);
  }
  sum(i, j) { return this.prefix[j] - this.prefix[i]; } // L[i:j]
}

const rs = new RangeSum([1, 2, 3, 4, 5]);
console.log(rs.sum(1, 3));
