// Day 700: Cheapest itinerary with at most k connections (k+1 flights).
// Approach: Bellman-Ford bounded to k+1 edges (relax with previous layer's dist),
// track predecessors to rebuild the route. Time O((k+1)*E), Space O(V).
function cheapest(flights, src, dst, k) {
  let dist = { [src]: 0 };
  let par = {};
  for (let it = 0; it <= k; it++) { // up to k+1 edges
    const nd = { ...dist };
    const np = { ...par };
    for (const [u, v, w] of flights) {
      if (u in dist) {
        const cand = dist[u] + w;
        if (!(v in nd) || cand < nd[v]) { nd[v] = cand; np[v] = u; }
      }
    }
    dist = nd; par = np;
  }
  if (!(dst in dist)) return [-1, []];
  const path = [];
  let cur = dst;
  while (cur !== src) { path.push(cur); cur = par[cur]; }
  path.push(src);
  return [dist[dst], path.reverse()];
}

const flights = [
  ["JFK", "ATL", 150], ["ATL", "SFO", 400], ["ORD", "LAX", 200],
  ["LAX", "DFW", 80], ["JFK", "HKG", 800], ["ATL", "ORD", 90],
  ["JFK", "LAX", 500],
];
const [cost, path] = cheapest(flights, "JFK", "LAX", 3);
console.log(path.join(" -> ") + `, costing $${cost}`);
// JFK -> ATL -> ORD -> LAX, costing $440
