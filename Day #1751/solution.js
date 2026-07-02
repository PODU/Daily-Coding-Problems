// Day 1751: Min steps to visit points in order on an 8-directional grid.
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
'use strict';

function minSteps(pts) {
  let total = 0;
  for (let i = 1; i < pts.length; i++) {
    const dx = Math.abs(pts[i][0] - pts[i - 1][0]);
    const dy = Math.abs(pts[i][1] - pts[i - 1][1]);
    total += Math.max(dx, dy);
  }
  return total;
}

const points = [[0, 0], [1, 1], [1, 2]];
console.log(minSteps(points)); // 2
