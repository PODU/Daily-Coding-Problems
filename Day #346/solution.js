// Day 346: Cheapest itinerary with up to k connections.
// Bellman-Ford limited to k+1 edges, tracking the path. Time O(k*E), Space O(V).
function cheapest(flights, src, dst, k) {
  let best = new Map([[src, { cost: 0, path: [src] }]]);
  for (let it = 0; it <= k; it++) {
    const nxt = new Map();
    for (const [key, val] of best) nxt.set(key, { cost: val.cost, path: val.path });
    for (const [u, v, w] of flights) {
      if (!best.has(u)) continue;
      const cost = best.get(u).cost + w;
      const cur = nxt.get(v);
      if (!cur || cost < cur.cost) nxt.set(v, { cost, path: [...best.get(u).path, v] });
    }
    best = nxt;
  }
  return best.get(dst);
}

function main() {
  const flights = [
    ['JFK', 'ATL', 150], ['ATL', 'SFO', 400], ['ORD', 'LAX', 200],
    ['LAX', 'DFW', 80], ['JFK', 'HKG', 800], ['ATL', 'ORD', 90], ['JFK', 'LAX', 500],
  ];
  const res = cheapest(flights, 'JFK', 'LAX', 3);
  console.log(`${res.path.join(' -> ')}, costing $${res.cost}.`);
}

main();
