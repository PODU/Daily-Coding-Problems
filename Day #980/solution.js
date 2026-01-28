// Count attacking bishop pairs by grouping on diagonals (row-col) and anti-diagonals (row+col).
// For each group of size k, add k*(k-1)/2. Time O(N), Space O(N).
function countAttackingPairs(bishops) {
  const diag = new Map(), anti = new Map();
  for (const [r, c] of bishops) {
    diag.set(r - c, (diag.get(r - c) || 0) + 1);
    anti.set(r + c, (anti.get(r + c) || 0) + 1);
  }
  let pairs = 0;
  for (const k of diag.values()) pairs += (k * (k - 1)) / 2;
  for (const k of anti.values()) pairs += (k * (k - 1)) / 2;
  return pairs;
}

const bishops = [[0, 0], [1, 2], [2, 2], [4, 0]];
console.log(countAttackingPairs(bishops)); // 2
