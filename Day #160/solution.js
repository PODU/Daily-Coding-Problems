// Day 160: Weighted tree diameter. One DFS; each node returns its longest
// downward branch, while we combine the top two branches. Time O(V+E).
'use strict';

function treeDiameter(edges, root) {
  const adj = new Map();
  const add = (u, v, w) => {
    if (!adj.has(u)) adj.set(u, []);
    adj.get(u).push([v, w]);
  };
  for (const [u, v, w] of edges) { add(u, v, w); add(v, u, w); }

  let best = 0;
  function dfs(node, parent) {
    let top1 = 0, top2 = 0;
    for (const [nb, w] of adj.get(node) || []) {
      if (nb === parent) continue;
      const d = w + dfs(nb, node);
      if (d > top1) { top2 = top1; top1 = d; }
      else if (d > top2) { top2 = d; }
    }
    best = Math.max(best, top1 + top2);
    return top1;
  }
  dfs(root, null);
  return best;
}

const edges = [
  ['a', 'b', 3], ['a', 'c', 5], ['a', 'd', 8],
  ['d', 'e', 2], ['d', 'f', 4],
  ['e', 'g', 1], ['e', 'h', 1],
];
console.log(treeDiameter(edges, 'a')); // 17
