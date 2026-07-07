// Unit converter as a graph: addRelation stores 1 a = factor b (edge a->b, b->a=1/factor).
// convert does BFS multiplying factors along the path. Time O(V+E) per query, Space O(V+E).
"use strict";

class UnitConverter {
  constructor() {
    this.adj = new Map();
  }
  _edge(a, b, f) {
    if (!this.adj.has(a)) this.adj.set(a, new Map());
    this.adj.get(a).set(b, f);
  }
  addRelation(a, b, factor) {
    this._edge(a, b, factor);
    this._edge(b, a, 1 / factor);
  }
  convert(qty, from, to) {
    if (from === to) return qty;
    if (!this.adj.has(from) || !this.adj.has(to)) return null;
    const dist = new Map([[from, qty]]);
    const q = [from];
    while (q.length) {
      const u = q.shift();
      for (const [nxt, f] of this.adj.get(u)) {
        if (!dist.has(nxt)) {
          const v = dist.get(u) * f;
          dist.set(nxt, v);
          if (nxt === to) return v;
          q.push(nxt);
        }
      }
    }
    return null;
  }
}

function main() {
  const uc = new UnitConverter();
  uc.addRelation("inches", "foot", 1 / 12);
  uc.addRelation("feet", "yard", 1 / 3);
  uc.addRelation("yards", "chain", 1 / 22);
  uc.addRelation("foot", "feet", 1);
  uc.addRelation("yard", "yards", 1);

  console.log(`1 yard = ${Math.round(uc.convert(1, "yard", "inches"))} inches`);
  console.log(`1 chain = ${Math.round(uc.convert(1, "chain", "inches"))} inches`);
  console.log(`1 chain = ${Math.round(uc.convert(1, "chain", "feet"))} feet`);
}

main();
