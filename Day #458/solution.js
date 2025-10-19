// Day 458: Validate directional (NE/SW/...) rules for consistency.
// Decompose into x/y strict orders; a cycle in either graph = contradiction.
// Time O(R + V) via DFS cycle detection.
function hasCycle(adj, nodes) {
  const color = new Map();
  for (const n of nodes) color.set(n, 0);
  function dfs(u) {
    color.set(u, 1);
    for (const v of adj.get(u) || []) {
      const cv = color.get(v);
      if (cv === 1) return true;
      if (cv === 0 && dfs(v)) return true;
    }
    color.set(u, 2);
    return false;
  }
  for (const n of nodes) if (color.get(n) === 0 && dfs(n)) return true;
  return false;
}

function validate(rules) {
  const gx = new Map(), gy = new Map(), nx = new Set(), ny = new Set();
  const less = (g, ns, small, big) => {
    if (!g.has(small)) g.set(small, []);
    g.get(small).push(big);
    ns.add(small); ns.add(big);
  };
  for (const r of rules) {
    const [a, d, b] = r.split(/\s+/);
    for (const c of d) {
      if (c === "N") less(gy, ny, b, a);
      else if (c === "S") less(gy, ny, a, b);
      else if (c === "E") less(gx, nx, b, a);
      else if (c === "W") less(gx, nx, a, b);
    }
  }
  return !hasCycle(gx, nx) && !hasCycle(gy, ny);
}

console.log(validate(["A N B", "B NE C", "C N A"]) ? "Valid" : "Invalid"); // Invalid
console.log(validate(["A NW B", "A N B"]) ? "Valid" : "Invalid"); // Valid
