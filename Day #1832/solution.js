// Expected rounds until one coin remains. Each round a coin survives w.p. 1/2.
// DP recurrence: E(n)*(2^n-1) = 2^n + sum_{k=2..n-1} C(n,k) E(k); E(0)=E(1)=0. O(n^2).
function expectedRounds(n) {
  if (n <= 1) return 0.0;
  const C = Array.from({ length: n + 1 }, () => new Array(n + 1).fill(0));
  for (let i = 0; i <= n; i++) {
    C[i][0] = 1;
    for (let j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
  }
  const E = new Array(n + 1).fill(0);
  for (let m = 2; m <= n; m++) {
    const pm = Math.pow(2, m);
    let sum = pm;
    for (let k = 2; k <= m - 1; k++) sum += C[m][k] * E[k];
    E[m] = sum / (pm - 1);
  }
  return E[n];
}

const n = 4;
console.log(expectedRounds(n)); // ~2.05714 rounds
