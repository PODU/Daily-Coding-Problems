// Unit conversion as a weighted graph; convert via BFS multiplying edge ratios.
// add_conversion(a, b, r): 1 a = r b. Time O(V + E) per query, Space O(V + E).
'use strict';

class UnitConverter {
  constructor() {
    this.graph = new Map();
  }

  addConversion(frm, to, factor) {
    // 1 frm = factor to
    if (!this.graph.has(frm)) this.graph.set(frm, new Map());
    if (!this.graph.has(to)) this.graph.set(to, new Map());
    this.graph.get(frm).set(to, factor);
    this.graph.get(to).set(frm, 1 / factor);
  }

  convert(qty, frm, to) {
    if (frm === to) return qty;
    if (!this.graph.has(frm) || !this.graph.has(to)) return null;
    const visited = new Set([frm]);
    const queue = [[frm, 1]];
    while (queue.length) {
      const [unit, ratio] = queue.shift();
      if (unit === to) return qty * ratio;
      for (const [nxt, f] of this.graph.get(unit)) {
        if (!visited.has(nxt)) {
          visited.add(nxt);
          queue.push([nxt, ratio * f]);
        }
      }
    }
    return null;
  }
}

function main() {
  const uc = new UnitConverter();
  uc.addConversion('foot', 'inch', 12);   // 12 inches = 1 foot
  uc.addConversion('yard', 'foot', 3);    // 3 feet = 1 yard
  uc.addConversion('chain', 'yard', 22);  // 22 yards = 1 chain

  console.log(`1 chain = ${Math.round(uc.convert(1, 'chain', 'inch'))} inches`);
  console.log(`1 yard = ${Math.round(uc.convert(1, 'yard', 'inch'))} inches`);
}

main();
