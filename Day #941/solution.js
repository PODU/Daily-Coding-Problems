// Day 941: Arbitrage exists iff a cycle has product of rates > 1, i.e. a negative cycle
// under weights w = -log(rate). Bellman-Ford. Time O(V^3), Space O(V).
function hasArbitrage(rate) {
  const n = rate.length;
  const dist = new Array(n).fill(0); // virtual source connected to all with weight 0
  for (let iter = 0; iter < n - 1; iter++)
    for (let u = 0; u < n; u++)
      for (let v = 0; v < n; v++) {
        const w = -Math.log(rate[u][v]);
        if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
      }
  for (let u = 0; u < n; u++)
    for (let v = 0; v < n; v++) {
      const w = -Math.log(rate[u][v]);
      if (dist[u] + w < dist[v] - 1e-12) return true;
    }
  return false;
}

const rate = [
  [1.0, 2.0, 1.0],
  [0.5, 1.0, 2.0],
  [1.0, 0.5, 1.0],
];
console.log(hasArbitrage(rate)); // true (0->1->2->0 = 4 > 1)
