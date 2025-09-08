// Point in polygon: ray-casting (even-odd rule). Boundary points are detected separately
// and return false. Time: O(N), Space: O(1).
function onSegment(px, py, ax, ay, bx, by) {
  const cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
  if (Math.abs(cross) > 1e-9) return false;
  return Math.min(ax, bx) - 1e-9 <= px && px <= Math.max(ax, bx) + 1e-9 &&
         Math.min(ay, by) - 1e-9 <= py && py <= Math.max(ay, by) + 1e-9;
}

function inside(poly, px, py) {
  const n = poly.length;
  let res = false;
  for (let i = 0, j = n - 1; i < n; j = i++) {
    const [xi, yi] = poly[i], [xj, yj] = poly[j];
    if (onSegment(px, py, xi, yi, xj, yj)) return false; // boundary
    if ((yi > py) !== (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi)
      res = !res;
  }
  return res;
}

const poly = [[0, 0], [4, 0], [4, 4], [0, 4]];
console.log(inside(poly, 2, 2)); // true
console.log(inside(poly, 4, 2)); // false (boundary)
console.log(inside(poly, 5, 5)); // false (outside)
