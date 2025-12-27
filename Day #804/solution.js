// Day 804: Cheapest fare from A to B with <= k connections (<= k+1 flights).
// Bellman-Ford limited to k+1 relaxation rounds, tracking parents for itinerary.
// Time O((k+1) * E), Space O(V + E).

function cheapest(flights, A, B, k) {
  const INF = Infinity;
  const dist = new Map();
  for (const [s, d] of flights) { dist.set(s, INF); dist.set(d, INF); }
  dist.set(A, 0);
  const parent = new Map();
  for (let it = 0; it <= k; it++) {
    const snap = new Map(dist);
    for (const [s, d, p] of flights) {
      if (snap.get(s) + p < dist.get(d)) {
        dist.set(d, snap.get(s) + p);
        parent.set(d, s);
      }
    }
  }
  if (dist.get(B) === INF) return [-1, []];
  const path = [];
  for (let c = B; ; c = parent.get(c)) {
    path.push(c);
    if (c === A) break;
  }
  path.reverse();
  return [dist.get(B), path];
}

const flights = [
  ["JFK", "ATL", 150], ["ATL", "SFO", 400], ["ORD", "LAX", 200],
  ["LAX", "DFW", 80], ["JFK", "HKG", 800], ["ATL", "ORD", 90],
  ["JFK", "LAX", 500],
];
const [cost, path] = cheapest(flights, "JFK", "LAX", 3);
console.log(`${path.join(" -> ")}, costing $${cost}`);
// JFK -> ATL -> ORD -> LAX, costing $440
