// Count attacking bishop pairs: group by diagonals d1=row+col, d2=row-col; sum k*(k-1)/2. Time O(N), Space O(N).
function countAttackingPairs(bishops) {
  const d1 = new Map(), d2 = new Map();
  for (const [r, c] of bishops) {
    d1.set(r + c, (d1.get(r + c) || 0) + 1);
    d2.set(r - c, (d2.get(r - c) || 0) + 1);
  }
  let pairs = 0;
  for (const k of d1.values()) pairs += (k * (k - 1)) / 2;
  for (const k of d2.values()) pairs += (k * (k - 1)) / 2;
  return pairs;
}

const bishops = [[0, 0], [1, 2], [2, 2], [4, 0]];
console.log(countAttackingPairs(bishops));
