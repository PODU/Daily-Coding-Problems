// Day 1120 - Minimum steps to cover points in order (8-directional moves)
// Cost between two points is Chebyshev distance max(|dx|,|dy|); sum them.
// Time: O(n), Space: O(1).

function minSteps(points) {
  let total = 0;
  for (let i = 1; i < points.length; i++) {
    const dx = Math.abs(points[i][0] - points[i - 1][0]);
    const dy = Math.abs(points[i][1] - points[i - 1][1]);
    total += Math.max(dx, dy);
  }
  return total;
}

console.log(minSteps([[0, 0], [1, 1], [1, 2]])); // 2
