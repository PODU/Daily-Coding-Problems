// Top-k similar website pairs by Jaccard similarity of user sets.
// Build per-site user sets, compute Jaccard for all pairs, sort desc (ties alpha), take k.
// Time: O(S^2 * U), Space: O(S*U).
function topKSimilar(visits, k) {
  const sites = new Map();
  for (const [site, user] of visits) {
    if (!sites.has(site)) sites.set(site, new Set());
    sites.get(site).add(user);
  }
  const names = [...sites.keys()].sort();
  const results = [];
  for (let i = 0; i < names.length; i++) {
    for (let j = i + 1; j < names.length; j++) {
      const A = sites.get(names[i]);
      const B = sites.get(names[j]);
      let inter = 0;
      for (const x of A) if (B.has(x)) inter++;
      const uni = A.size + B.size - inter;
      const sim = uni === 0 ? 0 : inter / uni;
      results.push([sim, names[i], names[j]]);
    }
  }
  results.sort((x, y) => {
    if (x[0] !== y[0]) return y[0] - x[0];
    if (x[1] !== y[1]) return x[1] < y[1] ? -1 : 1;
    return x[2] < y[2] ? -1 : 1;
  });
  return results.slice(0, k).map(r => [r[1], r[2]]);
}

const visits = [['a', 1], ['a', 3], ['a', 5], ['b', 2], ['b', 6],
  ['c', 1], ['c', 2], ['c', 3], ['c', 4], ['c', 5],
  ['d', 4], ['d', 5], ['d', 6], ['d', 7],
  ['e', 1], ['e', 3], ['e', 5], ['e', 6]];
const res = topKSimilar(visits, 1);
console.log('[' + res.map(p => `('${p[0]}', '${p[1]}')`).join(', ') + ']');
