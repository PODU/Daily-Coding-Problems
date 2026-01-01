// Minimum swaps to seat N couples side by side.
// Greedy: partner of p is p^1; swap mismatched partner into place. Time: O(N), Space: O(N).

function minSwaps(row) {
  row = row.slice();
  const n = row.length;
  const pos = new Array(n);
  for (let i = 0; i < n; i++) pos[row[i]] = i;
  let swaps = 0;
  for (let i = 0; i < n; i += 2) {
    const partner = row[i] ^ 1;
    if (row[i + 1] !== partner) {
      const j = pos[partner];
      pos[row[i + 1]] = j;
      pos[row[j]] = i + 1;
      [row[i + 1], row[j]] = [row[j], row[i + 1]];
      swaps++;
    }
  }
  return swaps;
}

console.log(minSwaps([0, 2, 1, 3]));
