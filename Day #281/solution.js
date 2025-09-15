// Day 281: Fewest bricks cut by a vertical line. Count prefix-sum edge positions;
// answer = rows - max edge frequency. Time O(total bricks), Space O(distinct edges).
function fewestCuts(wall) {
  const edge = new Map();
  let best = 0;
  for (const row of wall) {
    let sum = 0;
    for (let i = 0; i + 1 < row.length; i++) {
      sum += row[i];
      const c = (edge.get(sum) || 0) + 1;
      edge.set(sum, c);
      best = Math.max(best, c);
    }
  }
  return wall.length - best;
}

const wall = [
  [3, 5, 1, 1], [2, 3, 3, 2], [5, 5],
  [4, 4, 2], [1, 3, 3, 3], [1, 1, 6, 1, 1],
];
console.log(fewestCuts(wall)); // 2
