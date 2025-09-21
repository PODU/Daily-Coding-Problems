// Day 308: Count parenthesizations evaluating True. Interval DP. O(n^3) time, O(n^2) space.
function countTrue(e) {
  const n = e.length, V = (n + 1) >> 1;
  const T = Array.from({ length: V }, () => new Array(V).fill(0));
  const F = Array.from({ length: V }, () => new Array(V).fill(0));
  for (let i = 0; i < V; i++) {
    const val = e[2 * i] === "T";
    T[i][i] = val ? 1 : 0; F[i][i] = val ? 0 : 1;
  }
  for (let len = 2; len <= V; len++) for (let i = 0; i + len - 1 < V; i++) {
    const j = i + len - 1;
    for (let k = i; k < j; k++) {
      const op = e[2 * k + 1];
      const lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
      const total = (lt + lf) * (rt + rf);
      let t = 0;
      if (op === "&") t = lt * rt;
      else if (op === "|") t = lt * rt + lt * rf + lf * rt;
      else t = lt * rf + lf * rt;
      T[i][j] += t; F[i][j] += total - t;
    }
  }
  return T[0][V - 1];
}
console.log(countTrue(["F", "|", "T", "&", "T"])); // 2
