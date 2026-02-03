// Unit Converter: graph where edge a->b stores factor (1 a = f b).
// convert() does BFS multiplying factors. Time O(V+E) per query, Space O(V+E).
class UnitConverter {
  constructor() {
    this.g = new Map();
  }
  addUnit(from, to, factor) {
    if (!this.g.has(from)) this.g.set(from, []);
    if (!this.g.has(to)) this.g.set(to, []);
    this.g.get(from).push([to, factor]);
    this.g.get(to).push([from, 1.0 / factor]);
  }
  convert(value, from, to) {
    if (from === to) return value;
    const dist = new Map([[from, 1.0]]);
    const q = [from];
    while (q.length) {
      const u = q.shift();
      for (const [v, f] of (this.g.get(u) || [])) {
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

function main() {
  const uc = new UnitConverter();
  uc.addUnit("inch", "foot", 1.0 / 12);
  uc.addUnit("foot", "yard", 1.0 / 3);
  uc.addUnit("yard", "chain", 1.0 / 22);

  console.log(`1 chain = ${Math.round(uc.convert(1, "chain", "inch"))} inches`);
  console.log(`36 inches = ${Math.round(uc.convert(36, "inch", "yard"))} yards`);
}

main();
