// Day 1448: Fewest bricks cut by a vertical line through a brick wall.
// Count internal edge positions via prefix sums; answer = rows - maxEdgeCount.
// Time O(total bricks), Space O(distinct edges).
function leastBricks(wall) {
  const edges = new Map();
  let best = 0;
  for (const row of wall) {
    let pos = 0;
    for (let i = 0; i + 1 < row.length; i++) { // skip far right edge
      pos += row[i];
      const c = (edges.get(pos) || 0) + 1;
      edges.set(pos, c);
      if (c > best) best = c;
    }
  }
  return wall.length - best;
}

const wall = [
  [3, 5, 1, 1],
  [2, 3, 3, 2],
  [5, 5],
  [4, 4, 2],
  [1, 3, 3, 3],
  [1, 1, 6, 1, 1],
];
console.log(leastBricks(wall)); // 2
