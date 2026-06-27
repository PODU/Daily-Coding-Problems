// Day 1724: Cheapest itinerary with at most k connections (k flights/edges).
// Bellman-Ford limited to k relaxation rounds; track parents to reconstruct path.
// Time: O(k * E), Space: O(V).
'use strict';

function cheapestItinerary(flights, src, dst, k) {
  const INF = Infinity;
  const dist = new Map();
  const parent = new Map();
  for (const [u, v] of flights) {
    if (!dist.has(u)) dist.set(u, INF);
    if (!dist.has(v)) dist.set(v, INF);
  }
  dist.set(src, 0);

  // Relax all edges k times over a snapshot to bound edge count.
  for (let i = 0; i < k; i++) {
    const snap = new Map(dist);
    for (const [u, v, w] of flights) {
      const du = snap.get(u);
      if (du !== INF && du + w < dist.get(v)) {
        dist.set(v, du + w);
        parent.set(v, u);
      }
    }
  }

  if (dist.get(dst) === INF) return [null, null];
  const path = [];
  let at = dst;
  while (true) {
    path.push(at);
    if (at === src) break;
    at = parent.get(at);
  }
  path.reverse();
  return [path, dist.get(dst)];
}

function main() {
  const flights = [
    ['JFK', 'ATL', 150], ['ATL', 'SFO', 400], ['ORD', 'LAX', 200],
    ['LAX', 'DFW', 80],  ['JFK', 'HKG', 800], ['ATL', 'ORD', 90],
    ['JFK', 'LAX', 500],
  ];
  const [path, cost] = cheapestItinerary(flights, 'JFK', 'LAX', 3);
  console.log(`${path.join(' -> ')}, costing $${cost}`);
}

main();
