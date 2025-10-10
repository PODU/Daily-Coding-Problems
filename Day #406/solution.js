// Boolean parenthesization: count ways the expression evaluates to True.
// Interval DP storing #True/#False per substring. Time O(n^3), Space O(n^2).
function countTrue(a) {
  const val = [], op = [];
  for (let i = 0; i < a.length; i++) (i % 2 === 0 ? val : op).push(a[i]);
  const M = val.length;
  const T = Array.from({ length: M }, () => new Array(M).fill(0));
  const F = Array.from({ length: M }, () => new Array(M).fill(0));
  for (let i = 0; i < M; i++) { T[i][i] = val[i] === 'T' ? 1 : 0; F[i][i] = val[i] === 'F' ? 1 : 0; }
  for (let len = 2; len <= M; len++)
    for (let i = 0; i + len - 1 < M; i++) {
      const j = i + len - 1;
      for (let k = i; k < j; k++) {
        const o = op[k];
        const lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
        const tot = (lt + lf) * (rt + rf);
        if (o === '&') { T[i][j] += lt * rt; F[i][j] += tot - lt * rt; }
        else if (o === '|') { T[i][j] += tot - lf * rf; F[i][j] += lf * rf; }
        else { T[i][j] += lt * rf + lf * rt; F[i][j] += lt * rt + lf * rf; }
      }
    }
  return T[0][M - 1];
}

console.log(countTrue(['F', '|', 'T', '&', 'T']));
