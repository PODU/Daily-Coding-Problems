// Range sum via prefix sums: O(n) preprocessing, O(1) per query. sum(i,j) = pre[j]-pre[i].

class RangeSum {
  constructor(L) {
    this.pre = new Array(L.length + 1).fill(0);
    for (let k = 0; k < L.length; k++) this.pre[k + 1] = this.pre[k] + L[k];
  }
  sum(i, j) {
    return this.pre[j] - this.pre[i];
  }
}

const L = [1, 2, 3, 4, 5];
const rs = new RangeSum(L);
console.log(rs.sum(1, 3));
