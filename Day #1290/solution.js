// Day 1290: Strict point-in-polygon test (ray casting), boundary counts as outside.
// Check edges for on-boundary, then parity of rightward ray crossings. Time O(n), Space O(1).
function onSegment(a, b, p) {
  const cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
  if (Math.abs(cross) > 1e-9) return false;
  return p[0] >= Math.min(a[0], b[0]) - 1e-9 && p[0] <= Math.max(a[0], b[0]) + 1e-9 &&
         p[1] >= Math.min(a[1], b[1]) - 1e-9 && p[1] <= Math.max(a[1], b[1]) + 1e-9;
}

function inside(poly, p) {
  const n = poly.length;
  for (let i = 0; i < n; i++)
    if (onSegment(poly[i], poly[(i + 1) % n], p)) return false; // boundary
  let res = false;
  for (let i = 0, j = n - 1; i < n; j = i++) {
    const [xi, yi] = poly[i];
    const [xj, yj] = poly[j];
    if ((yi > p[1]) !== (yj > p[1])) {
      const xint = ((xj - xi) * (p[1] - yi)) / (yj - yi) + xi;
      if (p[0] < xint) res = !res;
    }
  }
  return res;
}

const square = [[0, 0], [4, 0], [4, 4], [0, 4]];
console.log(inside(square, [2, 2])); // true
console.log(inside(square, [5, 5])); // false
console.log(inside(square, [4, 2])); // false (boundary)
