// Day 958: reconstruct itinerary using every flight once, lexicographically smallest.
// Backtracking DFS over sorted adjacency. Worst O(E!) but fast in practice; Space O(E).

function itinerary(flights, start) {
  const adj = new Map();
  for (const [o, d] of flights) {
    if (!adj.has(o)) adj.set(o, []);
    adj.get(o).push(d);
  }
  for (const l of adj.values()) l.sort();
  const total = flights.length;
  const path = [start];

  function dfs(node) {
    if (path.length === total + 1) return true;
    const dests = adj.get(node) || [];
    for (let i = 0; i < dests.length; i++) {
      const d = dests[i];
      if (d === null) continue;
      dests[i] = null;
      path.push(d);
      if (dfs(d)) return true;
      path.pop();
      dests[i] = d;
    }
    return false;
  }

  return dfs(start) ? path : null;
}

function show(v) {
  return v === null ? "null" : "[" + v.map((x) => `'${x}'`).join(", ") + "]";
}

console.log(show(itinerary([["SFO","HKO"],["YYZ","SFO"],["YUL","YYZ"],["HKO","ORD"]], "YUL")));
console.log(show(itinerary([["SFO","COM"],["COM","YYZ"]], "COM")));
console.log(show(itinerary([["A","B"],["A","C"],["B","C"],["C","A"]], "A")));
