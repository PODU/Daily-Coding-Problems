// Closest pair of points via divide & conquer: sort by x, recurse, merge with strip check by y.
// O(n log n) time, O(n) space.
function dist(a, b) {
  return Math.hypot(a[0] - b[0], a[1] - b[1]);
}

function closestPair(points) {
  const px = points.slice().sort((a, b) => (a[0] - b[0]) || (a[1] - b[1]));
  let best = { d: Infinity, a: null, b: null };

  function consider(a, b) {
    const d = dist(a, b);
    if (d < best.d) best = { d, a, b };
  }

  // returns array sorted by y for [lo,hi)
  function rec(lo, hi) {
    const n = hi - lo;
    if (n <= 3) {
      const pts = px.slice(lo, hi);
      for (let i = 0; i < pts.length; i++)
        for (let j = i + 1; j < pts.length; j++) consider(pts[i], pts[j]);
      return pts.sort((a, b) => a[1] - b[1]);
    }
    const mid = lo + (n >> 1);
    const midx = px[mid][0];
    const left = rec(lo, mid);
    const right = rec(mid, hi);
    const merged = [];
    let i = 0, j = 0;
    while (i < left.length && j < right.length)
      merged.push(left[i][1] <= right[j][1] ? left[i++] : right[j++]);
    while (i < left.length) merged.push(left[i++]);
    while (j < right.length) merged.push(right[j++]);
    const strip = merged.filter((p) => Math.abs(p[0] - midx) < best.d);
    for (let a = 0; a < strip.length; a++)
      for (let b = a + 1; b < strip.length && (strip[b][1] - strip[a][1]) < best.d; b++)
        consider(strip[a], strip[b]);
    return merged;
  }

  rec(0, px.length);
  const pair = [best.a, best.b].sort((p, q) => (p[0] - q[0]) || (p[1] - q[1]));
  return pair;
}

const points = [[1, 1], [-1, -1], [3, 4], [6, 1], [-1, -6], [-4, -3]];
const [a, b] = closestPair(points);
console.log(`[(${a[0]}, ${a[1]}), (${b[0]}, ${b[1]})]`);
