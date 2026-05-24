// Alien dictionary: build edges from first differing chars of adjacent words,
// then Kahn's topological sort. Time O(total chars), Space O(letters + edges).
function alienOrder(words) {
  const adj = new Map();
  const indeg = new Map();
  for (const w of words)
    for (const c of w) {
      if (!adj.has(c)) adj.set(c, new Set());
      if (!indeg.has(c)) indeg.set(c, 0);
    }
  for (let i = 0; i + 1 < words.length; i++) {
    const a = words[i], b = words[i + 1];
    const n = Math.min(a.length, b.length);
    let j = 0;
    for (; j < n; j++) {
      if (a[j] !== b[j]) {
        if (!adj.get(a[j]).has(b[j])) {
          adj.get(a[j]).add(b[j]);
          indeg.set(b[j], indeg.get(b[j]) + 1);
        }
        break;
      }
    }
    if (j === n && a.length > b.length) return []; // invalid prefix
  }
  const heap = [...indeg.keys()].filter(c => indeg.get(c) === 0).sort();
  const order = [];
  while (heap.length) {
    heap.sort();
    const c = heap.shift();
    order.push(c);
    for (const nx of [...adj.get(c)].sort()) {
      indeg.set(nx, indeg.get(nx) - 1);
      if (indeg.get(nx) === 0) heap.push(nx);
    }
  }
  if (order.length !== indeg.size) return [];
  return order;
}

const words = ['xww', 'wxyz', 'wxyw', 'ywx', 'ywz'];
const order = alienOrder(words);
console.log("[" + order.map(c => `'${c}'`).join(", ") + "]");
