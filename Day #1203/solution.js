// Day 1203: Alien dictionary - derive letter order from sorted words.
// Build precedence graph from adjacent word diffs, Kahn topological sort. Time O(total chars), Space O(1) alphabet.
function alienOrder(words) {
  const adj = new Map(), indeg = new Map();
  for (const w of words) for (const c of w) {
    if (!adj.has(c)) { adj.set(c, new Set()); indeg.set(c, 0); }
  }
  for (let i = 0; i + 1 < words.length; i++) {
    const a = words[i], b = words[i + 1];
    const n = Math.min(a.length, b.length);
    for (let j = 0; j < n; j++) if (a[j] !== b[j]) {
      if (!adj.get(a[j]).has(b[j])) { adj.get(a[j]).add(b[j]); indeg.set(b[j], indeg.get(b[j]) + 1); }
      break;
    }
  }
  const q = [];
  for (const [c, d] of indeg) if (d === 0) q.push(c);
  const order = [];
  while (q.length) {
    const c = q.shift();
    order.push(c);
    for (const nx of [...adj.get(c)].sort()) {
      indeg.set(nx, indeg.get(nx) - 1);
      if (indeg.get(nx) === 0) q.push(nx);
    }
  }
  return order;
}

const words = ["xww", "wxyz", "wxyw", "ywx", "ywz"];
console.log(alienOrder(words)); // [ 'x', 'z', 'w', 'y' ]
