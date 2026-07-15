// Longest weighted path (diameter) in a tree via two DFS passes.
// DFS from any node finds one endpoint; DFS from it finds the other + path. Time: O(V+E). Space: O(V).
const g = {};
function add(a, b, w) {
  (g[a] ||= []).push([b, w]);
  (g[b] ||= []).push([a, w]);
}

function farthest(src) {
  const parent = {};
  let best = src, bestDist = 0;
  const visited = new Set();
  // iterative DFS
  const stack = [[src, 0, null]];
  while (stack.length) {
    const [u, d, par] = stack.pop();
    if (visited.has(u)) continue;
    visited.add(u);
    if (par !== null) parent[u] = par;
    if (d > bestDist) { bestDist = d; best = u; }
    for (const [v, w] of g[u]) if (!visited.has(v)) stack.push([v, d + w, u]);
  }
  return { node: best, dist: bestDist, parent };
}

add("a", "b", 3); add("a", "c", 5); add("a", "d", 8);
add("d", "e", 2); add("d", "f", 4); add("e", "g", 1); add("e", "h", 1);

const u = farthest("a").node;          // one endpoint
const res = farthest(u);               // other endpoint + parents
const v = res.node, len = res.dist, parent = res.parent;

const path = [];
for (let cur = v; ; cur = parent[cur]) { path.push(cur); if (cur === u) break; }

console.log(path.join(" -> ") + `, with a length of ${len}`);
// c -> a -> d -> f, with a length of 17
