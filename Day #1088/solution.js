// Boolean parenthesization via interval DP counting True/False groupings. Time O(n^3), Space O(n^2).
function countTrue(expr) {
  const m = expr.length;
  const n = (m + 1) >> 1;
  const val = [], ops = [];
  for (let i = 0; i < m; i++) {
    if (i % 2 === 0) val.push(expr[i] === 'T');
    else ops.push(expr[i]);
  }
  const T = Array.from({ length: n }, () => new Array(n).fill(0));
  const F = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let i = 0; i < n; i++) { T[i][i] = val[i] ? 1 : 0; F[i][i] = val[i] ? 0 : 1; }
  for (let len = 2; len <= n; len++)
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      for (let k = i; k < j; k++) {
        const op = ops[k];
        const lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
        if (op === '&') { T[i][j] += lt * rt; F[i][j] += lf * rf + lf * rt + lt * rf; }
        else if (op === '|') { T[i][j] += lt * rt + lt * rf + lf * rt; F[i][j] += lf * rf; }
        else { T[i][j] += lt * rf + lf * rt; F[i][j] += lt * rt + lf * rf; }
      }
    }
  return T[0][n - 1];
}

console.log(countTrue(['F', '|', 'T', '&', 'T']));
