// Day 1434: Cheapest flight A->B with at most k connections; print itinerary.
// Approach: Bellman-Ford relaxed k+1 times (k connections = k+1 edges), track parent.
// Time: O((k+1) * E), Space: O(V).

function cheapest(flights, A, B, k) {
  const dist = {};
  for (const [s, d] of flights) {
    if (!(s in dist)) dist[s] = Infinity;
    if (!(d in dist)) dist[d] = Infinity;
  }
  dist[A] = 0;
  let parent = {};
  for (let i = 0; i <= k; i++) {
    const cur = { ...dist };
    const curParent = { ...parent };
    for (const [s, d, price] of flights) {
      if (dist[s] !== Infinity && dist[s] + price < cur[d]) {
        cur[d] = dist[s] + price;
        curParent[d] = s;
      }
    }
    Object.assign(dist, cur);
    parent = curParent;
  }
  if (dist[B] === Infinity) return "No route";
  const path = [];
  let node = B;
  while (node !== A) { path.push(node); node = parent[node]; }
  path.push(A);
  path.reverse();
  return path.join(" -> ") + ", costing $" + dist[B];
}

const flights = [
  ['JFK', 'ATL', 150], ['ATL', 'SFO', 400], ['ORD', 'LAX', 200],
  ['LAX', 'DFW', 80], ['JFK', 'HKG', 800], ['ATL', 'ORD', 90], ['JFK', 'LAX', 500],
];
console.log(cheapest(flights, 'JFK', 'LAX', 3));
