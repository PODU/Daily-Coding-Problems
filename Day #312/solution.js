// Day 312: Tilings of a 2xN board with dominoes & L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3). O(N) time.
function tilings(n) {
  if (n === 0) return 1;
  if (n === 1) return 1;
  if (n === 2) return 2;
  const f = new Array(n + 1).fill(0);
  f[0] = 1; f[1] = 1; f[2] = 2;
  for (let i = 3; i <= n; i++) f[i] = 2 * f[i - 1] + f[i - 3];
  return f[n];
}
console.log(tilings(4)); // 11
