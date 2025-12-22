// Day 780: Topological sort of courses (prereqs map). DFS post-order with
// cycle detection; returns null if a cycle exists. O(V + E).
function courseOrder(graph) {
  const state = new Map(); // 0/undef=unseen, 1=visiting, 2=done
  const order = [];

  function dfs(c) {
    state.set(c, 1);
    for (const pre of (graph[c] || [])) {
      const s = state.get(pre) || 0;
      if (s === 1) return false;
      if (s === 0 && !dfs(pre)) return false;
    }
    state.set(c, 2);
    order.push(c);
    return true;
  }

  for (const c of Object.keys(graph).sort()) {
    if ((state.get(c) || 0) === 0 && !dfs(c)) return null;
  }
  return order;
}

const g = { CSC300: ["CSC100", "CSC200"], CSC200: ["CSC100"], CSC100: [] };
console.log(courseOrder(g)); // [ 'CSC100', 'CSC200', 'CSC300' ]
