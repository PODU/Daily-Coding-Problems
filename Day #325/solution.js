// Unit conversion via weighted graph; addConversion adds a<->b edges, convert does BFS multiplying ratios.
// Time: O(V+E) per query, Space: O(V+E).
class UnitConverter {
  constructor() {
    this.adj = new Map();
  }
  addConversion(a, b, ratio) {
    if (!this.adj.has(a)) this.adj.set(a, []);
    if (!this.adj.has(b)) this.adj.set(b, []);
    this.adj.get(a).push([b, ratio]);
    this.adj.get(b).push([a, 1 / ratio]);
  }
  convert(value, from, to) {
    if (from === to) return value;
    const dist = new Map([[from, value]]);
    const q = [from];
    while (q.length) {
      const u = q.shift();
      for (const [v, w] of this.adj.get(u) || []) {
        if (!dist.has(v)) {
          dist.set(v, dist.get(u) * w);
          if (v === to) return dist.get(v);
          q.push(v);
        }
      }
    }
    return dist.has(to) ? dist.get(to) : NaN;
  }
}

function main() {
  const uc = new UnitConverter();
  uc.addConversion("foot", "inch", 12);
  uc.addConversion("yard", "foot", 3);
  uc.addConversion("chain", "yard", 22);
  const r = uc.convert(1, "yard", "inch");
  console.log(`1 yard = ${r.toFixed(1)} inch`);
}

main();
