// Day 1762: Count parenthesizations of a boolean expression evaluating to True.
// Interval DP over operands: t[i][j]/f[i][j] = #ways range evals True/False,
// combine across each split operator. Time O(n^3), Space O(n^2).
function countTrue(tokens) {
  const vals = [], ops = [];
  tokens.forEach((tk, i) => (i % 2 === 0 ? vals : ops).push(tk));
  const n = vals.length;
  const t = Array.from({ length: n }, () => Array(n).fill(0));
  const f = Array.from({ length: n }, () => Array(n).fill(0));
  for (let i = 0; i < n; i++) {
    t[i][i] = vals[i] === 'T' ? 1 : 0;
    f[i][i] = vals[i] === 'F' ? 1 : 0;
  }
  for (let len = 2; len <= n; len++)
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      for (let k = i; k < j; k++) {
        const op = ops[k];
        const lt = t[i][k], lf = f[i][k], rt = t[k + 1][j], rf = f[k + 1][j];
        const tot = (lt + lf) * (rt + rf);
        if (op === '&') { t[i][j] += lt * rt; f[i][j] += tot - lt * rt; }
        else if (op === '|') { f[i][j] += lf * rf; t[i][j] += tot - lf * rf; }
        else { t[i][j] += lt * rf + lf * rt; f[i][j] += lt * rt + lf * rf; }
      }
    }
  return t[0][n - 1];
}

console.log(countTrue(['F', '|', 'T', '&', 'T']));
