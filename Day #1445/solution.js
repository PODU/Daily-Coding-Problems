// Day 1445: Unit converter. Model units as a weighted graph (edge = conversion
// factor) and BFS for a path on query. addUnit O(1); convert O(V+E).
class UnitConverter {
  constructor() { this.graph = new Map(); } // a -> Map(b -> factor); 1 a = factor b

  addUnit(from, to, factor) {
    if (!this.graph.has(from)) this.graph.set(from, new Map());
    if (!this.graph.has(to)) this.graph.set(to, new Map());
    this.graph.get(from).set(to, factor);
    this.graph.get(to).set(from, 1 / factor);
  }

  convert(value, from, to) {
    if (from === to) return value;
    const dist = new Map([[from, 1]]);
    const q = [from];
    while (q.length) {
      const u = q.shift();
      for (const [v, f] of this.graph.get(u) || []) {
        if (!dist.has(v)) {
          dist.set(v, dist.get(u) * f);
          if (v === to) return value * dist.get(v);
          q.push(v);
        }
      }
    }
    return NaN;
  }
}

const uc = new UnitConverter();
uc.addUnit("foot", "inch", 12);
uc.addUnit("yard", "foot", 3);
uc.addUnit("chain", "yard", 22);
console.log(Math.round(uc.convert(1, "yard", "inch"))); // 36
