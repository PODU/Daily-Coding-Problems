// Day 1023: Alien dictionary - order of letters from sorted words.
// Approach: build precedence graph from adjacent words, Kahn's topological sort.
// Time O(total chars + V + E), Space O(V + E).
function alienOrder(words) {
  const appear = [];
  const adj = new Map();
  const indeg = new Map();
  for (const w of words)
    for (const c of w)
      if (!indeg.has(c)) {
        indeg.set(c, 0);
        appear.push(c);
        adj.set(c, new Set());
      }

  for (let i = 0; i + 1 < words.length; i++) {
    const a = words[i], b = words[i + 1];
    const n = Math.min(a.length, b.length);
    for (let j = 0; j < n; j++) {
      if (a[j] !== b[j]) {
        if (!adj.get(a[j]).has(b[j])) {
          adj.get(a[j]).add(b[j]);
          indeg.set(b[j], indeg.get(b[j]) + 1);
        }
        break;
      }
    }
  }

  const q = appear.filter((c) => indeg.get(c) === 0);
  const res = [];
  while (q.length) {
    const c = q.shift();
    res.push(c);
    for (const nb of [...adj.get(c)].sort()) {
      indeg.set(nb, indeg.get(nb) - 1);
      if (indeg.get(nb) === 0) q.push(nb);
    }
  }
  return res;
}

const words = ["xww", "wxyz", "wxyw", "ywx", "ywz"];
console.log("[" + alienOrder(words).map((c) => `'${c}'`).join(", ") + "]");
