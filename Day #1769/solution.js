// Uphill-then-downhill shortest cyclic route from home (node 0): Dijkstra over
// states (node, phase). UP edges require rising elevation, DOWN edges require
// falling; a free phase switch (the peak) is allowed at non-home nodes.
// Time: O(E log V), Space: O(V+E).  (Binary-heap priority queue.)
class MinHeap {
  constructor() { this.a = []; }
  push(x) {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p][0] <= a[i][0]) break;
      [a[p], a[i]] = [a[i], a[p]];
      i = p;
    }
  }
  pop() {
    const a = this.a;
    const top = a[0];
    const last = a.pop();
    if (a.length) {
      a[0] = last;
      let i = 0;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < a.length && a[l][0] < a[s][0]) s = l;
        if (r < a.length && a[r][0] < a[s][0]) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]];
        i = s;
      }
    }
    return top;
  }
  get size() { return this.a.length; }
}

function shortestRoute(elev, paths, home = 0) {
  const n = Object.keys(elev).length;
  const adj = Array.from({ length: n }, () => []);
  for (const [key, w] of Object.entries(paths)) {
    const [u, v] = key.split(",").map(Number);
    adj[u].push([v, w]);
  }
  const INF = Infinity;
  const dist = new Array(2 * n).fill(INF);
  const pq = new MinHeap();
  dist[home * 2] = 0;
  pq.push([0, home * 2]);
  while (pq.size) {
    const [d, s] = pq.pop();
    if (d > dist[s]) continue;
    const node = s >> 1, phase = s & 1;
    if (phase === 0 && node !== home) {
      const ns = node * 2 + 1;
      if (d < dist[ns]) { dist[ns] = d; pq.push([d, ns]); }
    }
    for (const [v, w] of adj[node]) {
      if (phase === 0 && elev[v] > elev[node]) {
        const ns = v * 2;
        if (d + w < dist[ns]) { dist[ns] = d + w; pq.push([d + w, ns]); }
      }
      if (phase === 1 && elev[v] < elev[node]) {
        const ns = v * 2 + 1;
        if (d + w < dist[ns]) { dist[ns] = d + w; pq.push([d + w, ns]); }
      }
    }
  }
  return dist[home * 2 + 1];
}

const elevations = { 0: 5, 1: 25, 2: 15, 3: 20, 4: 10 };
const paths = {
  "0,1": 10, "0,2": 8, "0,3": 15, "1,3": 12,
  "2,4": 10, "3,4": 5, "3,0": 17, "4,0": 10,
};
console.log(shortestRoute(elevations, paths));
