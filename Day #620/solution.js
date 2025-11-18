// Brick wall: hashmap of prefix-sum edge positions (excluding full-width edge).
// Answer = numRows - maxEdgeCount. Time O(total bricks), Space O(distinct edges).
function leastBricks(wall) {
  const edges = new Map();
  let maxCount = 0;
  for (const row of wall) {
    let sum = 0;
    for (let i = 0; i + 1 < row.length; i++) {
      sum += row[i];
      const c = (edges.get(sum) || 0) + 1;
      edges.set(sum, c);
      if (c > maxCount) maxCount = c;
    }
  }
  return wall.length - maxCount;
}

const wall = [[3, 5, 1, 1], [2, 3, 3, 2], [5, 5], [4, 4, 2], [1, 3, 3, 3], [1, 1, 6, 1, 1]];
console.log(leastBricks(wall));
