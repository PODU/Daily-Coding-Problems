// Arbitrage detection: edge weight = -log(rate); negative cycle => arbitrage. Bellman-Ford.
// Time: O(V*E) = O(V^3), Space: O(V).
function hasArbitrage(rates) {
  const n = rates.length;
  const dist = new Array(n).fill(0.0); // all 0: detect any reachable negative cycle
  for (let k = 0; k < n - 1; k++)
    for (let u = 0; u < n; u++)
      for (let v = 0; v < n; v++) {
        const w = -Math.log(rates[u][v]);
        if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
      }
  for (let u = 0; u < n; u++)
    for (let v = 0; v < n; v++) {
      const w = -Math.log(rates[u][v]);
      if (dist[u] + w < dist[v] - 1e-12) return true;
    }
  return false;
}

const rates = [
  [1.0, 2.0, 1.0],
  [0.5, 1.0, 4.0],
  [1.0, 0.25, 1.0],
];
console.log(`Arbitrage exists: ${hasArbitrage(rates)}`);
