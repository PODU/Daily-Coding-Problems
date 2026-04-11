// Bishops attack along diagonals: group by (row-col) and (row+col); each group of k adds k*(k-1)/2. O(N) time, O(N) space.

function countAttackingPairs(M, bishops) {
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

function main() {
  const M = 5;
  const bishops = [[0, 0], [1, 2], [2, 2], [4, 0]];
  console.log(countAttackingPairs(M, bishops));
}

main();
