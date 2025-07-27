// Reconstruct Itinerary: backtracking DFS over lexicographically sorted edges.
// First complete itinerary using all edges (tried in lex order) is the answer.
// Time: exponential worst case; Space: O(E).
function reconstruct(flights, start) {
  const adj = {};
  for (const [a, b] of flights) (adj[a] = adj[a] || []).push(b);
  for (const k in adj) adj[k].sort();
  const used = {};
  for (const k in adj) used[k] = new Array(adj[k].length).fill(false);
  const total = flights.length;
  const path = [start];

  function dfs(node) {
    if (path.length === total + 1) return true;
    const dests = adj[node];
    if (!dests) return false;
    const u = used[node];
    for (let i = 0; i < dests.length; i++) {
      if (u[i]) continue;
      u[i] = true;
      path.push(dests[i]);
      if (dfs(dests[i])) return true;
      path.pop();
      u[i] = false;
    }
    return false;
  }
  return dfs(start) ? path : null;
}

function fmt(v) {
  if (v === null) return "null";
  return "[" + v.map(x => "'" + x + "'").join(", ") + "]";
}

console.log(fmt(reconstruct([['SFO','HKO'],['YYZ','SFO'],['YUL','YYZ'],['HKO','ORD']], 'YUL')));
console.log(fmt(reconstruct([['SFO','COM'],['COM','YYZ']], 'COM')));
console.log(fmt(reconstruct([['A','B'],['A','C'],['B','C'],['C','A']], 'A')));
