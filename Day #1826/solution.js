// Dijkstra from node 0; answer is the max shortest-path distance (broadcast time).
// O((V+E) log V) with a binary heap.
class MinHeap {
  constructor() { this.a = []; }
  push(x) { const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] <= a[i][0]) break;[a[p], a[i]] = [a[i], a[p]]; i = p; } }
  pop() { const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) { a[0] = last; let i = 0;
      for (;;) { let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < a.length && a[l][0] < a[s][0]) s = l;
        if (r < a.length && a[r][0] < a[s][0]) s = r;
        if (s === i) break;[a[s], a[i]] = [a[i], a[s]]; i = s; } }
    return top; }
  get size() { return this.a.length; }
}

function broadcastTime(edges) {
  let maxNode = 0;
  for (const [a, b] of edges) maxNode = Math.max(maxNode, a, b);
  const V = maxNode + 1;
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
      if (d + w < dist[v]) { dist[v] = d + w; pq.push([dist[v], v]); }
    }
  }
  return Math.max(...dist);
}

const edges = [[0, 1, 5], [0, 2, 3], [0, 5, 4], [1, 3, 8], [2, 3, 1], [3, 5, 10], [3, 4, 5]];
console.log(broadcastTime(edges)); // 9
