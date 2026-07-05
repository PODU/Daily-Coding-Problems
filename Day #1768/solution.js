// Direction-rule consistency: decompose each rule into strict x/y "greater-than"
// edges; a contradiction is a cycle in either axis graph (DFS cycle detection).
// Time: O(V+E) per axis, Space: O(V+E).
class Checker {
  constructor() {
    this.gx = new Map();
    this.gy = new Map();
    this.nodes = new Set();
  }
  edge(g, a, b) {
    if (!g.has(a)) g.set(a, []);
    g.get(a).push(b);
    this.nodes.add(a);
    this.nodes.add(b);
  }
  addRule(a, dir, b) {
    for (const c of dir) {
      if (c === "N") this.edge(this.gy, a, b);
      else if (c === "S") this.edge(this.gy, b, a);
      else if (c === "E") this.edge(this.gx, a, b);
      else if (c === "W") this.edge(this.gx, b, a);
    }
  }
  hasCycle(g) {
    const state = new Map();
    const dfs = (u) => {
      state.set(u, 1);
      for (const v of g.get(u) || []) {
        const s = state.get(v) || 0;
        if (s === 1) return true;
        if (s === 0 && dfs(v)) return true;
      }
      state.set(u, 2);
      return false;
    };
    for (const n of this.nodes)
      if ((state.get(n) || 0) === 0 && dfs(n)) return true;
    return false;
  }
  validates() {
    return !this.hasCycle(this.gx) && !this.hasCycle(this.gy);
  }
}

const c1 = new Checker();
c1.addRule("A", "N", "B");
c1.addRule("B", "NE", "C");
c1.addRule("C", "N", "A");
if (!c1.validates())
  console.log("does not validate, since A cannot be both north and south of C.");

const c2 = new Checker();
c2.addRule("A", "NW", "B");
c2.addRule("A", "N", "B");
if (c2.validates()) console.log("A NW B / A N B is considered valid.");
