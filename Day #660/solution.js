// Alien dictionary: build edges from first differing char of adjacent words,
// then Kahn's BFS topological sort. Time O(C + V + E), Space O(V + E).

function alienOrder(words) {
  const graph = new Map();
  const indeg = new Map();
  for (const w of words)
    for (const c of w) {
      if (!graph.has(c)) { graph.set(c, new Set()); indeg.set(c, 0); }
    }
  for (let i = 0; i + 1 < words.length; i++) {
    const a = words[i], b = words[i + 1];
    const n = Math.min(a.length, b.length);
    for (let j = 0; j < n; j++) {
      if (a[j] !== b[j]) {
        if (!graph.get(a[j]).has(b[j])) {
          graph.get(a[j]).add(b[j]);
          indeg.set(b[j], indeg.get(b[j]) + 1);
        }
        break;
      }
    }
  }
  const queue = [...graph.keys()].filter((c) => indeg.get(c) === 0).sort();
  const order = [];
  while (queue.length) {
    const c = queue.shift();
    order.push(c);
    const nexts = [...graph.get(c)].sort();
    for (const nxt of nexts) {
      indeg.set(nxt, indeg.get(nxt) - 1);
      if (indeg.get(nxt) === 0) {
        queue.push(nxt);
        queue.sort();
      }
    }
  }
  return order;
}

function main() {
  const words = ['xww', 'wxyz', 'wxyw', 'ywx', 'ywz'];
  const order = alienOrder(words);
  console.log("[" + order.map((c) => "'" + c + "'").join(", ") + "]");
}

main();
