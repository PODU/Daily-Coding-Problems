// Day 1206: Validate directional (N/S/E/W) rules for consistency.
// Split into vertical & horizontal strict-order graphs; a cycle = contradiction. Time O(V+E), Space O(V+E).
function add(g, u, v) {
  if (!g.has(u)) g.set(u, []);
  if (!g.has(v)) g.set(v, []);
  g.get(u).push(v);
}

function hasCycle(g) {
  const color = new Map();
  function dfs(u) {
    color.set(u, 1);
    for (const v of g.get(u)) {
      const c = color.get(v) || 0;
      if (c === 1) return true;
      if (c === 0 && dfs(v)) return true;
    }
    color.set(u, 2);
    return false;
  }
  for (const u of g.keys()) if ((color.get(u) || 0) === 0 && dfs(u)) return true;
  return false;
}

function validate(rules) {
  const gy = new Map(), gx = new Map();
  for (const [a, d, b] of rules) {
    if (d.includes("N")) add(gy, a, b);
    if (d.includes("S")) add(gy, b, a);
    if (d.includes("E")) add(gx, a, b);
    if (d.includes("W")) add(gx, b, a);
  }
  return !hasCycle(gy) && !hasCycle(gx);
}

const rules = [["A", "N", "B"], ["B", "NE", "C"], ["C", "N", "A"]];
console.log(validate(rules) ? "validates" : "does not validate"); // does not validate
