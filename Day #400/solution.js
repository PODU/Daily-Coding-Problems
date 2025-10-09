// Prefix-sum array P (P[k]=sum of first k elems); sum(i,j)=P[j]-P[i]. Preprocess O(n), query O(1).
class RangeSum {
  constructor(L) {
    this.P = new Array(L.length + 1).fill(0);
    for (let k = 0; k < L.length; k++) this.P[k + 1] = this.P[k] + L[k];
  }
  sum(i, j) { return this.P[j] - this.P[i]; }
}

const L = [1, 2, 3, 4, 5];
const rs = new RangeSum(L);
console.log(rs.sum(1, 3));
