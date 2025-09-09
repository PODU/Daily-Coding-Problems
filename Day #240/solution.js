// Couples holding hands: union the two couples occupying each seat-pair. Minimum swaps =
// N - (number of connected components among couples). Time: O(N alpha), Space: O(N).
function minSwaps(row) {
  const n = row.length / 2;
  const parent = Array.from({length: n}, (_, i) => i);
  const find = x => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };
  let comps = n;
  for (let i = 0; i < row.length; i += 2) {
    const a = find(Math.floor(row[i] / 2)), b = find(Math.floor(row[i + 1] / 2));
    if (a !== b) { parent[a] = b; comps--; }
  }
  return n - comps;
}

console.log(minSwaps([0, 2, 1, 3])); // 1
