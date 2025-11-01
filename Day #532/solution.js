// Count attacking bishop pairs: group by diagonals r+c and r-c, sum C(size,2).
// Time O(N), Space O(N).
function countAttackingPairs(bishops) {
  const diag1 = new Map();
  const diag2 = new Map();
  for (const [r, c] of bishops) {
    diag1.set(r + c, (diag1.get(r + c) || 0) + 1);
    diag2.set(r - c, (diag2.get(r - c) || 0) + 1);
  }
  let pairs = 0;
  for (const cnt of diag1.values()) pairs += (cnt * (cnt - 1)) / 2;
  for (const cnt of diag2.values()) pairs += (cnt * (cnt - 1)) / 2;
  return pairs;
}

const bishops = [
  [0, 0],
  [1, 2],
  [2, 2],
  [4, 0],
];
console.log(countAttackingPairs(bishops));
