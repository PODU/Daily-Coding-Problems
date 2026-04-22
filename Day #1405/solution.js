// Interval DP: T[i][j]/F[i][j] = #ways subexpr of operands i..j is True/False.
// Split at each operator, combine child counts per &,|,^. Time O(n^3), Space O(n^2).

function countTrue(expr) {
  const vals = [], ops = [];
  for (let i = 0; i < expr.length; i++)
    (i % 2 === 0 ? vals : ops).push(expr[i]);
  const n = vals.length;
  if (n === 0) return 0;
  const T = Array.from({ length: n }, () => new Array(n).fill(0));
  const F = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let i = 0; i < n; i++) {
    T[i][i] = vals[i] === 'T' ? 1 : 0;
    F[i][i] = vals[i] === 'F' ? 1 : 0;
  }
  for (let len = 2; len <= n; len++)
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      for (let k = i; k < j; k++) {
        const op = ops[k];
        const lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
        const tot = (lt + lf) * (rt + rf);
        let t;
        if (op === '&') t = lt * rt;
        else if (op === '|') t = lt * rt + lt * rf + lf * rt;
        else t = lt * rf + lf * rt;
        T[i][j] += t;
        F[i][j] += tot - t;
      }
    }
  return T[0][n - 1];
}

console.log(countTrue(['F', '|', 'T', '&', 'T'])); // 2
