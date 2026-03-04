// Day 1153: Min steps to visit points in order (8-directional moves).
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
function minSteps(pts) {
  let total = 0;
  for (let i = 1; i < pts.length; i++)
    total += Math.max(Math.abs(pts[i][0] - pts[i - 1][0]), Math.abs(pts[i][1] - pts[i - 1][1]));
  return total;
}

console.log(minSteps([[0, 0], [1, 1], [1, 2]])); // 2
