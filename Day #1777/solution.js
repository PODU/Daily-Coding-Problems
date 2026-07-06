// Count attacking bishop pairs: group by diagonal (r-c) and anti-diagonal (r+c), sum c*(c-1)/2.
// Time: O(N), Space: O(N).
function countAttackingPairs(M, bishops) {
  const diag = new Map(), anti = new Map();
  for (const [r, c] of bishops) {
    diag.set(r - c, (diag.get(r - c) || 0) + 1);
    anti.set(r + c, (anti.get(r + c) || 0) + 1);
  }
  let pairs = 0;
  for (const v of diag.values()) pairs += (v * (v - 1)) / 2;
  for (const v of anti.values()) pairs += (v * (v - 1)) / 2;
  return pairs;
}

const M = 5;
const bishops = [[0, 0], [1, 2], [2, 2], [4, 0]];
console.log(countAttackingPairs(M, bishops));
