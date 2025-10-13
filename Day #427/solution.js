// Day 427: Shortest uphill-then-downhill route from/to home (location 0).
// State Dijkstra: each node split into up/down phases; switch at the peak.
// Up edges need strictly higher elevation, down edges strictly lower. Time O((V+E)logV).
function solve(elev, edges, n) {
  const adj = Array.from({ length: n }, () => []);
  for (const [u, v, w] of edges) adj[u].push([v, w]);
  const home = 0, S = n * 2; // state = node*2 + phase (0 up, 1 down)
  const dist = new Array(S).fill(Infinity), prev = new Array(S).fill(-1);
  dist[home * 2] = 0;

  // binary min-heap of [dist, state]
  const pq = [[0, home * 2]];
  const push = (d, s) => {
    pq.push([d, s]);
    let i = pq.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (pq[p][0] <= pq[i][0]) break;
      [pq[p], pq[i]] = [pq[i], pq[p]];
      i = p;
    }
  };
  const pop = () => {
    const top = pq[0], last = pq.pop();
    if (pq.length) {
      pq[0] = last;
      let i = 0;
      for (;;) {
        let l = 2 * i + 1, r = 2 * i + 2, m = i;
        if (l < pq.length && pq[l][0] < pq[m][0]) m = l;
        if (r < pq.length && pq[r][0] < pq[m][0]) m = r;
        if (m === i) break;
        [pq[m], pq[i]] = [pq[i], pq[m]];
        i = m;
      }
    }
    return top;
  };

  while (pq.length) {
    const [d, s] = pop();
    if (d > dist[s]) continue;
    const u = Math.floor(s / 2), ph = s % 2;
    if (ph === 0 && u !== home) {
      const ns = u * 2 + 1;
      if (d < dist[ns]) { dist[ns] = d; prev[ns] = s; push(d, ns); }
    }
    for (const [v, w] of adj[u]) {
      let ns;
      if (ph === 0 && elev[v] > elev[u]) ns = v * 2;
      else if (ph === 1 && elev[v] < elev[u]) ns = v * 2 + 1;
      else continue;
      if (d + w < dist[ns]) { dist[ns] = d + w; prev[ns] = s; push(d + w, ns); }
    }
  }
  const goal = home * 2 + 1;
  const nodes = [];
  for (let cur = goal; cur !== -1; cur = prev[cur]) nodes.push(Math.floor(cur / 2));
  nodes.reverse();
  const path = [];
  for (const x of nodes)
    if (path.length === 0 || path[path.length - 1] !== x) path.push(x);
  console.log(path.join(" -> ") + `, distance ${dist[goal]}`);
}

const elev = { 0: 5, 1: 25, 2: 15, 3: 20, 4: 10 };
const edges = [[0, 1, 10], [0, 2, 8], [0, 3, 15], [1, 3, 12],
               [2, 4, 10], [3, 4, 5], [3, 0, 17], [4, 0, 10]];
solve(elev, edges, 5);
