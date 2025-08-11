// Day 100: 8-directional steps between two points = Chebyshev distance
// max(|dx|,|dy|). Sum over consecutive points. O(n) time.
function minSteps(pts) {
  let total = 0;
  for (let i = 1; i < pts.length; i++)
    total += Math.max(Math.abs(pts[i][0] - pts[i - 1][0]),
                      Math.abs(pts[i][1] - pts[i - 1][1]));
  return total;
}

console.log(minSteps([[0, 0], [1, 1], [1, 2]])); // 2
