// Arbitrage detection: weight=-log(rate), Bellman-Ford finds a negative-weight cycle => arbitrage. O(V^3) (V*E edges, V-1 passes).
'use strict';

function hasArbitrage(rates) {
  const n = rates.length;
  const dist = new Array(n).fill(0); // virtual source: all start at 0
  for (let it = 0; it < n - 1; it++) {
    for (let u = 0; u < n; u++)
      for (let v = 0; v < n; v++) {
        const w = -Math.log(rates[u][v]);
        if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
      }
  }
  for (let u = 0; u < n; u++)
    for (let v = 0; v < n; v++) {
      const w = -Math.log(rates[u][v]);
      if (dist[u] + w < dist[v] - 1e-12) return true;
    }
  return false;
}

const arb = [[1, 0.5, 0.2], [2, 1, 0.5], [5, 2, 1]];
const consistent = [[1, 2, 4], [0.5, 1, 2], [0.25, 0.5, 1]];
console.log(hasArbitrage(arb) ? "true" : "false");
console.log(hasArbitrage(consistent) ? "true" : "false");
