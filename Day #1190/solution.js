// Dijkstra from node 0 over nodes 0..N (undirected); answer = max finite shortest-path distance.
// Time: O(E log V), Space: O(V + E). (Uses a simple binary min-heap.)
"use strict";

class MinHeap {
  constructor() { this.a = []; }
  push(x) {
    const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] <= a[i][0]) break; [a[p], a[i]] = [a[i], a[p]]; i = p; }
  }
  pop() {
    const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) { a[0] = last; let i = 0; const n = a.length;
      while (true) { let l = 2*i+1, r = 2*i+2, s = i;
        if (l < n && a[l][0] < a[s][0]) s = l;
        if (r < n && a[r][0] < a[s][0]) s = r;
        if (s === i) break; [a[s], a[i]] = [a[i], a[s]]; i = s; } }
    return top;
  }
  get size() { return this.a.length; }
}

function networkDelay(N, edges) {
  const V = N + 1;
  const adj = Array.from({ length: V }, () => []);
  for (const [a, b, t] of edges) { adj[a].push([b, t]); adj[b].push([a, t]); }
  const dist = new Array(V).fill(Infinity);
  dist[0] = 0;
  const pq = new MinHeap();
  pq.push([0, 0]);
  while (pq.size) {
    const [d, u] = pq.pop();
    if (d > dist[u]) continue;
    for (const [v, w] of adj[u]) {
      if (dist[u] + w < dist[v]) { dist[v] = dist[u] + w; pq.push([dist[v], v]); }
    }
  }
  let ans = 0;
  for (const d of dist) if (d !== Infinity) ans = Math.max(ans, d);
  return ans;
}

const N = 5;
const edges = [[0,1,5],[0,2,3],[0,5,4],[1,3,8],[2,3,1],[3,5,10],[3,4,5]];
console.log(networkDelay(N, edges));
