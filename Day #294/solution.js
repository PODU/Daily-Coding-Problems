// Uphill-then-downhill closed route: Dijkstra on up-edges from 0, Dijkstra on reversed down-edges
// to 0; answer = min over peaks of distUp[m]+distDown[m]. Time O((V+E)logV), Space O(V+E).
function dijkstra(n, adj, src) {
  const d = new Array(n).fill(Infinity);
  d[src] = 0;
  // simple array-based priority queue (small graph)
  const pq = [[0, src]];
  while (pq.length) {
    pq.sort((a, b) => a[0] - b[0]);
    const [du, u] = pq.shift();
    if (du > d[u]) continue;
    for (const [v, w] of adj[u]) {
      if (d[u] + w < d[v]) { d[v] = d[u] + w; pq.push([d[v], v]); }
    }
  }
  return d;
}

const n = 5;
const elev = { 0: 5, 1: 25, 2: 15, 3: 20, 4: 10 };
const paths = { "0,1": 10, "0,2": 8, "0,3": 15, "1,3": 12, "2,4": 10, "3,4": 5, "3,0": 17, "4,0": 10 };
const up = Array.from({ length: n }, () => []);
const downRev = Array.from({ length: n }, () => []);
for (const key in paths) {
  const [u, v] = key.split(",").map(Number);
  const w = paths[key];
  if (elev[v] > elev[u]) up[u].push([v, w]);
  if (elev[v] < elev[u]) downRev[v].push([u, w]);
}
const distUp = dijkstra(n, up, 0);
const distDown = dijkstra(n, downRev, 0);
let best = Infinity;
for (let m = 1; m < n; m++) {
  if (distUp[m] < Infinity && distDown[m] < Infinity) best = Math.min(best, distUp[m] + distDown[m]);
}
console.log(best);
