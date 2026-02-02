// Day 1004: Range sum query sum(i, j) = L[i:j].
// Pre-process a prefix-sum array (O(N)); each query is prefix[j]-prefix[i] in O(1).

class RangeSum {
  constructor(L) {
    this.prefix = new Array(L.length + 1).fill(0);
    for (let i = 0; i < L.length; i++) this.prefix[i + 1] = this.prefix[i] + L[i];
  }
  sum(i, j) {
    return this.prefix[j] - this.prefix[i];
  }
}

const rs = new RangeSum([1, 2, 3, 4, 5]);
console.log(rs.sum(1, 3)); // 5
