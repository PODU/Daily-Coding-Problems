// Day 439: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian-path algorithm with sorted adjacency. O(E log E).

function findItinerary(flights, start) {
  const graph = new Map();
  for (const [o, d] of flights) {
    if (!graph.has(o)) graph.set(o, []);
    graph.get(o).push(d);
  }
  // sort descending so pop() yields the lexicographically smallest next hop
  for (const list of graph.values()) list.sort((a, b) => (a < b ? 1 : a > b ? -1 : 0));

  const route = [];
  const stack = [start];
  while (stack.length) {
    const u = stack[stack.length - 1];
    const list = graph.get(u);
    if (list && list.length) {
      stack.push(list.pop());
    } else {
      route.push(stack.pop());
    }
  }
  route.reverse();
  if (route.length !== flights.length + 1) return null;
  return route;
}

console.log(findItinerary(
  [['SFO','HKO'],['YYZ','SFO'],['YUL','YYZ'],['HKO','ORD']], 'YUL'));
// [ 'YUL', 'YYZ', 'SFO', 'HKO', 'ORD' ]
console.log(findItinerary([['SFO','COM'],['COM','YYZ']], 'COM')); // null
console.log(findItinerary([['A','B'],['A','C'],['B','C'],['C','A']], 'A'));
// [ 'A', 'B', 'C', 'A', 'C' ]
