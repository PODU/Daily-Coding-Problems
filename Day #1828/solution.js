// Arbitrage = negative cycle in graph with weights -log(rate). Bellman-Ford.
// O(V^3) for a dense rate table.
function hasArbitrage(rate) {
  const n = rate.length;
  const w = rate.map(row => row.map(r => -Math.log(r)));
  const dist = new Array(n).fill(0); // virtual source, all 0
  for (let it = 0; it < n - 1; it++)
    for (let u = 0; u < n; u++)
      for (let v = 0; v < n; v++)
        if (dist[u] + w[u][v] < dist[v]) dist[v] = dist[u] + w[u][v];
  for (let u = 0; u < n; u++)
    for (let v = 0; v < n; v++)
      if (dist[u] + w[u][v] < dist[v] - 1e-12) return true;
  return false;
}

const rate = [
  [1.0, 0.8, 0.5],
  [1.3, 1.0, 1.9],
  [1.9, 0.5, 1.0],
];
console.log(hasArbitrage(rate) ? "true" : "false"); // true
