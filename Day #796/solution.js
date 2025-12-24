// Day 796: Point strictly inside a polygon.
// Ray-casting (even-odd rule) + on-boundary check. Time O(N), Space O(1).

function onSegment(a, b, p) {
  const cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
  if (Math.abs(cross) > 1e-9) return false;
  return Math.min(a[0], b[0]) - 1e-9 <= p[0] && p[0] <= Math.max(a[0], b[0]) + 1e-9 &&
         Math.min(a[1], b[1]) - 1e-9 <= p[1] && p[1] <= Math.max(a[1], b[1]) + 1e-9;
}

function insidePolygon(poly, p) {
  const n = poly.length;
  for (let i = 0; i < n; i++)
    if (onSegment(poly[i], poly[(i + 1) % n], p)) return false;
  let inside = false;
  for (let i = 0, j = n - 1; i < n; j = i++) {
    if (((poly[i][1] > p[1]) !== (poly[j][1] > p[1])) &&
        (p[0] < (poly[j][0] - poly[i][0]) * (p[1] - poly[i][1]) /
                (poly[j][1] - poly[i][1]) + poly[i][0]))
      inside = !inside;
  }
  return inside;
}

const square = [[0, 0], [4, 0], [4, 4], [0, 4]];
console.log(insidePolygon(square, [2, 2])); // true
console.log(insidePolygon(square, [4, 2])); // false (boundary)
console.log(insidePolygon(square, [5, 5])); // false
