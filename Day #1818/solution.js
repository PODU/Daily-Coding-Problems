// Closest pair of points via divide-and-conquer on x, strip-merge on y.
// Each point tagged with an id for an unambiguous left/right split. Time: O(n log n). Space: O(n).
function dist2(a, b) {
  const dx = a[0] - b[0], dy = a[1] - b[1];
  return dx * dx + dy * dy;
}

function rec(px, py) {
  const n = px.length;
  if (n <= 3) {
    let best = Infinity, bp = [px[0], px[0]];
    for (let i = 0; i < n; i++)
      for (let j = i + 1; j < n; j++)
        if (dist2(px[i], px[j]) < best) { best = dist2(px[i], px[j]); bp = [px[i], px[j]]; }
    return bp;
  }
  const mid = n >> 1;
  const midX = px[mid][0];
  const lx = px.slice(0, mid), rx = px.slice(mid);
  const leftIds = new Set(lx.map((p) => p[2]));
  const ly = py.filter((p) => leftIds.has(p[2]));
  const ry = py.filter((p) => !leftIds.has(p[2]));

  const bl = rec(lx, ly), br = rec(rx, ry);
  let best = dist2(bl[0], bl[1]) < dist2(br[0], br[1]) ? bl : br;
  let d2 = dist2(best[0], best[1]);

  const strip = py.filter((p) => (p[0] - midX) ** 2 < d2);
  for (let i = 0; i < strip.length; i++)
    for (let j = i + 1; j < strip.length && (strip[j][1] - strip[i][1]) ** 2 < d2; j++)
      if (dist2(strip[i], strip[j]) < d2) { d2 = dist2(strip[i], strip[j]); best = [strip[i], strip[j]]; }
  return best;
}

function closestPair(points) {
  const pts = points.map((p, i) => [p[0], p[1], i]);
  const px = [...pts].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const py = [...pts].sort((a, b) => a[1] - b[1] || a[0] - b[0]);
  const r = rec(px, py);
  const pair = [[r[0][0], r[0][1]], [r[1][0], r[1][1]]];
  pair.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return pair;
}

const pts = [[1, 1], [-1, -1], [3, 4], [6, 1], [-1, -6], [-4, -3]];
const res = closestPair(pts);
console.log(`[(${res[0][0]}, ${res[0][1]}), (${res[1][0]}, ${res[1][1]})]`);
// [(-1, -1), (1, 1)]
