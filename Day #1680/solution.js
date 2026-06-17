// Day 1680: Strict point-in-polygon. Reject boundary via on-segment test, else
// ray-casting parity. Time O(N), Space O(1).
function onSeg(x1, y1, x2, y2, px, py) {
  const cross = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);
  if (Math.abs(cross) > 1e-9) return false;
  return px >= Math.min(x1, x2) - 1e-9 && px <= Math.max(x1, x2) + 1e-9 &&
         py >= Math.min(y1, y2) - 1e-9 && py <= Math.max(y1, y2) + 1e-9;
}

function inside(poly, px, py) {
  const n = poly.length;
  for (let i = 0; i < n; i++) {
    const [x1, y1] = poly[i], [x2, y2] = poly[(i + 1) % n];
    if (onSeg(x1, y1, x2, y2, px, py)) return false;
  }
  let res = false;
  for (let i = 0, j = n - 1; i < n; j = i++) {
    const [xi, yi] = poly[i], [xj, yj] = poly[j];
    if ((yi > py) !== (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi)
      res = !res;
  }
  return res;
}

const sq = [[0, 0], [4, 0], [4, 4], [0, 4]];
console.log(inside(sq, 2, 2)); // true
console.log(inside(sq, 4, 2)); // false
console.log(inside(sq, 5, 5)); // false
