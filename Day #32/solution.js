// Currency arbitrage: weights = -log(rate), Bellman-Ford detects negative cycle. O(V*E).
function hasArbitrage(rates) {
  const n = rates.length;
  const w = rates.map(row => row.map(x => -Math.log(x)));
  const dist = new Array(n).fill(0); // virtual super-source reaching all nodes at 0
  for (let it = 0; it < n - 1; it++)
    for (let u = 0; u < n; u++)
      for (let v = 0; v < n; v++)
        if (dist[u] + w[u][v] < dist[v] - 1e-12) dist[v] = dist[u] + w[u][v];
  for (let u = 0; u < n; u++)
    for (let v = 0; v < n; v++)
      if (dist[u] + w[u][v] < dist[v] - 1e-12) return true;
  return false;
}

const r1 = [[1.0, 0.7, 0.5], [1.4, 1.0, 0.7], [2.1, 1.4, 1.0]];
const r2 = [[1.0, 2.0, 4.0], [0.5, 1.0, 2.0], [0.25, 0.5, 1.0]];
console.log(hasArbitrage(r1));
console.log(hasArbitrage(r2));
