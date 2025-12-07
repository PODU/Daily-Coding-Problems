// Day 711: Point strictly inside polygon. First reject boundary via on-segment
// test, then ray-casting parity test. Time O(N) per query.
function onSeg(a, b, p) {
  const cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
  if (Math.abs(cross) > 1e-9) return false;
  return Math.min(a[0], b[0]) - 1e-9 <= p[0] && p[0] <= Math.max(a[0], b[0]) + 1e-9 &&
         Math.min(a[1], b[1]) - 1e-9 <= p[1] && p[1] <= Math.max(a[1], b[1]) + 1e-9;
}

function inside(poly, p) {
  const n = poly.length;
  for (let i = 0; i < n; i++)
    if (onSeg(poly[i], poly[(i + 1) % n], p)) return false;
  let inq = false;
  for (let i = 0, j = n - 1; i < n; j = i++) {
    if ((poly[i][1] > p[1]) !== (poly[j][1] > p[1])) {
      const xint = (poly[j][0] - poly[i][0]) * (p[1] - poly[i][1]) /
                   (poly[j][1] - poly[i][1]) + poly[i][0];
      if (p[0] < xint) inq = !inq;
    }
  }
  return inq;
}

const sq = [[0, 0], [4, 0], [4, 4], [0, 4]];
console.log(inside(sq, [2, 2]) ? "True" : "False");
console.log(inside(sq, [4, 2]) ? "True" : "False");
console.log(inside(sq, [5, 5]) ? "True" : "False");
