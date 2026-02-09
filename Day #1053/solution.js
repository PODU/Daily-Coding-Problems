// Day 1053: Directional consistency. Decompose each rule into strict x/y
// inequalities, build a directed "greater-than" graph per axis, and detect a
// cycle (contradiction) via DFS. Time O(V+E), Space O(V+E).

function add(g, u, v) {
  if (!g.has(u)) g.set(u, new Set());
  if (!g.has(v)) g.set(v, new Set());
  g.get(u).add(v);
}

function hasCycle(g) {
  const state = new Map(); // 0 = visiting, 1 = done
  function dfs(u) {
    state.set(u, 0);
    for (const w of g.get(u) || []) {
      if (state.get(w) === 0) return true;
      if (!state.has(w) && dfs(w)) return true;
    }
    state.set(u, 1);
    return false;
  }
  for (const n of g.keys()) if (!state.has(n) && dfs(n)) return true;
  return false;
}

function validate(rules) {
  const gx = new Map(), gy = new Map();
  for (const rule of rules) {
    const [a, d, b] = rule.split(/\s+/);
    for (const ch of d) {
      if (ch === 'N') add(gy, a, b);
      else if (ch === 'S') add(gy, b, a);
      else if (ch === 'E') add(gx, a, b);
      else if (ch === 'W') add(gx, b, a);
    }
  }
  return !(hasCycle(gx) || hasCycle(gy));
}

const ex1 = ["A N B", "B NE C", "C N A"];
const ex2 = ["A NW B", "A N B"];
console.log(!validate(ex1)
  ? "does not validate, since A cannot be both north and south of C."
  : "is considered valid.");
console.log(validate(ex2) ? "is considered valid." : "does not validate.");
