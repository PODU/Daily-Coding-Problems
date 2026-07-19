// Day 1841: Top-k most similar website pairs by Jaccard similarity of their visitor sets.
// Time O(W^2 * U) over W websites; Space O(total pairs).

function topKSimilar(pairs, k) {
  const sites = {};
  for (const [w, u] of pairs) (sites[w] ??= new Set()).add(u);

  const names = Object.keys(sites).sort();
  const scored = [];
  for (let i = 0; i < names.length; i++)
    for (let j = i + 1; j < names.length; j++) {
      const A = sites[names[i]],
        B = sites[names[j]];
      let inter = 0;
      for (const x of A) if (B.has(x)) inter++;
      const union = A.size + B.size - inter;
      const jac = union ? inter / union : 0;
      scored.push([jac, names[i], names[j]]);
    }
  scored.sort((a, b) => b[0] - a[0]);
  return scored.slice(0, k).map(([, a, b]) => [a, b]);
}

function main() {
  const pairs = [
    ["a", 1], ["a", 3], ["a", 5],
    ["b", 2], ["b", 6],
    ["c", 1], ["c", 2], ["c", 3], ["c", 4], ["c", 5],
    ["d", 4], ["d", 5], ["d", 6], ["d", 7],
    ["e", 1], ["e", 3], ["e", 5], ["e", 6],
  ];
  const res = topKSimilar(pairs, 1);
  console.log("[" + res.map(([a, b]) => `('${a}', '${b}')`).join(", ") + "]");
}

main();
