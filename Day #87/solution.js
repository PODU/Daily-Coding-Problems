// Day 87: Validate directional rules. N/S -> vertical graph, E/W -> horizontal graph,
// edge u->v means u strictly greater on that axis. A contradiction is a cycle. Time O(V+E).
function edge(adj, a, b) {
  if (!adj.has(a)) adj.set(a, new Set());
  if (!adj.has(b)) adj.set(b, new Set());
  adj.get(a).add(b);
}

function hasCycle(adj) {
  const state = new Map(); // 0=unvisited, 1=visiting, 2=done
  function dfs(u) {
    state.set(u, 1);
    for (const v of adj.get(u)) {
      const s = state.get(v) || 0;
      if (s === 1) return true;
      if (s === 0 && dfs(v)) return true;
    }
    state.set(u, 2);
    return false;
  }
  for (const u of adj.keys())
    if ((state.get(u) || 0) === 0 && dfs(u)) return true;
  return false;
}

function validate(rules) {
  const vert = new Map(), horiz = new Map();
  for (const [a, dir, b] of rules) {
    for (const c of dir) {
      if (c === "N") edge(vert, a, b);
      else if (c === "S") edge(vert, b, a);
      else if (c === "E") edge(horiz, a, b);
      else if (c === "W") edge(horiz, b, a);
    }
  }
  return !hasCycle(vert) && !hasCycle(horiz);
}

const rules = [["A", "N", "B"], ["B", "NE", "C"], ["C", "N", "A"]];
console.log(validate(rules) ? "validates" : "does not validate");
// does not validate
