// Day 856: Top k similar website pairs.
// Approach: Jaccard similarity (|A∩B| / |A∪B|) over user sets, take top k pairs.
// Time: O(W^2 * U), Space: O(W * U).
'use strict';

function topKSimilar(visits, k) {
  const sites = new Map();
  for (const [site, user] of visits) {
    if (!sites.has(site)) sites.set(site, new Set());
    sites.get(site).add(user);
  }
  const names = [...sites.keys()].sort();
  const scored = [];
  for (let i = 0; i < names.length; i++)
    for (let j = i + 1; j < names.length; j++) {
      const A = sites.get(names[i]), B = sites.get(names[j]);
      let inter = 0;
      for (const u of A) if (B.has(u)) inter++;
      const uni = A.size + B.size - inter;
      const sim = uni ? inter / uni : 0;
      scored.push([sim, [names[i], names[j]]]);
    }
  scored.sort((a, b) => b[0] - a[0]);
  return scored.slice(0, k).map((x) => x[1]);
}

const visits = [['a', 1], ['a', 3], ['a', 5],
  ['b', 2], ['b', 6],
  ['c', 1], ['c', 2], ['c', 3], ['c', 4], ['c', 5],
  ['d', 4], ['d', 5], ['d', 6], ['d', 7],
  ['e', 1], ['e', 3], ['e', 5], ['e', 6]];
const res = topKSimilar(visits, 1);
console.log('[' + res.map((p) => `('${p[0]}', '${p[1]}')`).join(', ') + ']');
