// Day 416: Min king-moves to visit points in order = sum of Chebyshev distances max(|dx|,|dy|).
// Time O(n), Space O(1).
function minSteps(pts) {
  let total = 0;
  for (let i = 1; i < pts.length; i++) {
    const dx = Math.abs(pts[i][0] - pts[i - 1][0]);
    const dy = Math.abs(pts[i][1] - pts[i - 1][1]);
    total += Math.max(dx, dy);
  }
  return total;
}

const pts = [[0, 0], [1, 1], [1, 2]];
console.log(minSteps(pts));
