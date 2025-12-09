// Day 722: Minimum swaps so each couple sits side by side.
// Approach: Greedy - fix each even seat; partner = x^1. Each swap places one couple.
// Answer equals N - (#cycles). Time: O(N), Space: O(N).

function minSwaps(row) {
  row = row.slice();
  const pos = new Map();
  row.forEach((v, i) => pos.set(v, i));
  let swaps = 0;
  for (let i = 0; i < row.length; i += 2) {
    const partner = row[i] ^ 1;
    if (row[i + 1] !== partner) {
      const j = pos.get(partner);
      pos.set(row[i + 1], j);
      pos.set(partner, i + 1);
      [row[i + 1], row[j]] = [row[j], row[i + 1]];
      swaps++;
    }
  }
  return swaps;
}

console.log("Minimum swaps:", minSwaps([0, 2, 1, 3])); // 1
