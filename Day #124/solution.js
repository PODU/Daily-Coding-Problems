// Day 124: Expected flipping rounds until one coin remains.
// DP: E[n](1-2^-n) = 1 + sum_{k<n} P(k survive)*E[k]. O(n^2) time, O(n) space.
function expectedRounds(n) {
  const E = new Array(n + 1).fill(0);
  for (let m = 2; m <= n; m++) {
    let p = Math.pow(0.5, m); // p_0
    let s = 0;
    for (let k = 0; k < m; k++) {
      s += p * E[k];
      p = (p * (m - k)) / (k + 1);
    }
    const pn = Math.pow(0.5, m);
    E[m] = (1 + s) / (1 - pn);
  }
  return n >= 1 ? E[n] : 0;
}

for (const n of [1, 2, 3, 4, 5, 10]) {
  console.log(`n=${String(n).padEnd(2)} expected rounds = ${expectedRounds(n).toFixed(6)}`);
}
