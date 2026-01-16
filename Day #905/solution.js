// Word-chain circle = Eulerian circuit on directed graph (node=letter, edge first->last).
// Check in==out degree for all nodes and single SCC over non-zero-degree nodes. O(V+E).
function canChain(words) {
  const indeg = {}, outdeg = {}, adj = {}, radj = {};
  const nodes = new Set();
  for (const w of words) {
    const a = w[0], b = w[w.length - 1];
    outdeg[a] = (outdeg[a] || 0) + 1;
    indeg[b] = (indeg[b] || 0) + 1;
    (adj[a] = adj[a] || []).push(b);
    (radj[b] = radj[b] || []).push(a);
    nodes.add(a); nodes.add(b);
  }
  for (const n of nodes) if ((indeg[n] || 0) !== (outdeg[n] || 0)) return false;
  if (nodes.size === 0) return true;
  const start = nodes.values().next().value;

  function dfs(g, u, seen) {
    seen.add(u);
    for (const v of (g[u] || [])) if (!seen.has(v)) dfs(g, v, seen);
  }

  let seen = new Set();
  dfs(adj, start, seen);
  for (const n of nodes) if (!seen.has(n)) return false;
  seen = new Set();
  dfs(radj, start, seen);
  for (const n of nodes) if (!seen.has(n)) return false;
  return true;
}

const words = ["chair", "height", "racket", "touch", "tunic"];
console.log(canChain(words) ? "True" : "False");
