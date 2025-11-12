// Group users into a set per site; for every site pair compute Jaccard = |A&B|/|A|B|.
// Sort by similarity DESC, tie-break by pair lexicographically; take top k. Time O(P^2 * U).
"use strict";

function topKSimilar(visits, k) {
  const sites = new Map();
  for (const [site, user] of visits) {
    if (!sites.has(site)) sites.set(site, new Set());
    sites.get(site).add(user);
  }
  const names = [...sites.keys()].sort();
  const cands = [];
  for (let i = 0; i < names.length; i++) {
    for (let j = i + 1; j < names.length; j++) {
      const a = sites.get(names[i]);
      const b = sites.get(names[j]);
      let inter = 0;
      for (const u of a) if (b.has(u)) inter++;
      const uni = a.size + b.size - inter;
      const sim = uni === 0 ? 0 : inter / uni;
      cands.push([sim, names[i], names[j]]);
    }
  }
  cands.sort((p, q) => {
    if (p[0] !== q[0]) return q[0] - p[0];
    if (p[1] !== q[1]) return p[1] < q[1] ? -1 : 1;
    return p[2] < q[2] ? -1 : p[2] > q[2] ? 1 : 0;
  });
  return cands.slice(0, k).map(([, x, y]) => [x, y]);
}

const visits = [
  ['a', 1], ['a', 3], ['a', 5],
  ['b', 2], ['b', 6],
  ['c', 1], ['c', 2], ['c', 3], ['c', 4], ['c', 5],
  ['d', 4], ['d', 5], ['d', 6], ['d', 7],
  ['e', 1], ['e', 3], ['e', 5], ['e', 6],
];

const result = topKSimilar(visits, 1);
const formatted =
  "[" + result.map(([x, y]) => `('${x}', '${y}')`).join(", ") + "]";
console.log(formatted);
