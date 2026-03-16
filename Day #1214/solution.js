// Day 1214: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian path over sorted multigraph adjacency. Time O(E log E), Space O(E).
function findItinerary(flights, start) {
  const adj = new Map();
  for (const [u, v] of flights) {
    if (!adj.has(u)) adj.set(u, []);
    adj.get(u).push(v);
  }
  for (const list of adj.values()) list.sort().reverse(); // pop() gives smallest
  const total = flights.length;
  const route = [], st = [start];
  while (st.length) {
    const u = st[st.length - 1];
    const list = adj.get(u);
    if (list && list.length) st.push(list.pop());
    else route.push(st.pop());
  }
  route.reverse();
  return route.length === total + 1 ? route : null;
}

const flights = [["SFO", "HKO"], ["YYZ", "SFO"], ["YUL", "YYZ"], ["HKO", "ORD"]];
console.log(findItinerary(flights, "YUL"));
// [ 'YUL', 'YYZ', 'SFO', 'HKO', 'ORD' ]
