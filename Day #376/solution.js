// Closest coin by Manhattan distance. Linear scan.
// Time: O(n), Space: O(1).
function closestCoin(me, coins) {
  let best = null, bestD = Infinity;
  for (const c of coins) {
    const d = Math.abs(c[0] - me[0]) + Math.abs(c[1] - me[1]);
    if (d < bestD) { bestD = d; best = c; }
  }
  return best;
}

const me = [0, 2];
const coins = [[0, 4], [1, 0], [2, 0], [3, 2]];
const b = closestCoin(me, coins);
console.log(`(${b[0]}, ${b[1]})`);
