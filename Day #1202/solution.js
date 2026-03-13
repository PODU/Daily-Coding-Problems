// Day 1202: Minimum swaps so couples sit side by side.
// Greedy: for each even seat, swap partner of its occupant into the next seat. Time O(N), Space O(N).
function minSwaps(row) {
  row = row.slice();
  const pos = new Array(row.length);
  for (let i = 0; i < row.length; i++) pos[row[i]] = i;
  let swaps = 0;
  for (let i = 0; i < row.length; i += 2) {
    const partner = row[i] ^ 1;
    if (row[i + 1] !== partner) {
      const j = pos[partner];
      [pos[row[i + 1]], pos[row[j]]] = [pos[row[j]], pos[row[i + 1]]];
      [row[i + 1], row[j]] = [row[j], row[i + 1]];
      swaps++;
    }
  }
  return swaps;
}

console.log(minSwaps([0, 2, 1, 3])); // 1
