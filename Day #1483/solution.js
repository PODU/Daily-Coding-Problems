// Day 1483: Shortest closed route from home (0) strictly ascending then
// descending. up[v]: shortest ascending 0->v; down[v]: shortest descending v->0
// (Dijkstra from 0 on reversed descending graph). Answer = min up[v]+down[v].
// Time O((V+E) log V) with a binary heap; here a simple O(V^2) Dijkstra suffices.

const INF = Infinity;

function dijkstra(n, adj, src) {
  const dist = new Array(n).fill(INF);
  const pred = new Array(n).fill(-1);
  const done = new Array(n).fill(false);
  dist[src] = 0;
  for (let it = 0; it < n; ++it) {
    let u = -1;
    for (let i = 0; i < n; ++i) if (!done[i] && (u === -1 || dist[i] < dist[u])) u = i;
    if (u === -1 || dist[u] === INF) break;
    done[u] = true;
    for (const [v, w] of adj[u]) {
      if (dist[u] + w < dist[v]) { dist[v] = dist[u] + w; pred[v] = u; }
    }
  }
  return [dist, pred];
}

function shortestRoute(elevations, paths) {
  const n = elevations.length;
  const upAdj = Array.from({ length: n }, () => []);
  const revDesc = Array.from({ length: n }, () => []);
  for (const [[a, b], w] of paths) {
    if (elevations[b] > elevations[a]) upAdj[a].push([b, w]);
    else if (elevations[b] < elevations[a]) revDesc[b].push([a, w]);
  }
  const [up, upPred] = dijkstra(n, upAdj, 0);
  const [down, downPred] = dijkstra(n, revDesc, 0);

  let best = INF, peak = -1;
  for (let v = 1; v < n; ++v)
    if (up[v] > 0 && down[v] > 0 && up[v] + down[v] < best) { best = up[v] + down[v]; peak = v; }

  const route = [];
  const upPath = [];
  for (let c = peak; c !== -1; c = upPred[c]) upPath.push(c);
  upPath.reverse();
  route.push(...upPath);
  for (let c = downPred[peak]; c !== -1; c = downPred[c]) route.push(c);
  return [best, route];
}

const elevations = [5, 25, 15, 20, 10];
const paths = [
  [[0, 1], 10], [[0, 2], 8], [[0, 3], 15], [[1, 3], 12],
  [[2, 4], 10], [[3, 4], 5], [[3, 0], 17], [[4, 0], 10],
];
const [dist, route] = shortestRoute(elevations, paths);
console.log(`The shortest valid path would be ${route.join(" -> ")}, with a distance of ${dist}.`);
// The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
