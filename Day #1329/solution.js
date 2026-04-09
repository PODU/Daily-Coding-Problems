// Day 1329: Closest pair of points via divide & conquer. O(n log n) time.
// Sort by x, recurse on halves, then check the middle strip ordered by y (each point vs next ~7).

function dist(a, b) {
  return Math.hypot(a[0] - b[0], a[1] - b[1]);
}

function closest(pts) {
  const px = pts.slice().sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const py = pts.slice().sort((a, b) => a[1] - b[1]);

  function rec(px, py) {
    const n = px.length;
    if (n <= 3) {
      let best = [Infinity, null, null];
      for (let i = 0; i < n; i++)
        for (let j = i + 1; j < n; j++) {
          const d = dist(px[i], px[j]);
          if (d < best[0]) best = [d, px[i], px[j]];
        }
      return best;
    }
    const mid = n >> 1;
    const pivot = px[mid];
    const midx = pivot[0];
    const lx = px.slice(0, mid), rx = px.slice(mid);
    const ly = [], ry = [];
    for (const p of py) {
      if (p[0] < pivot[0] || (p[0] === pivot[0] && p[1] < pivot[1])) ly.push(p);
      else ry.push(p);
    }
    let best = rec(lx, ly);
    const r = rec(rx, ry);
    if (r[0] < best[0]) best = r;
    const strip = py.filter((p) => Math.abs(p[0] - midx) < best[0]);
    for (let i = 0; i < strip.length; i++)
      for (let j = i + 1; j < strip.length && strip[j][1] - strip[i][1] < best[0]; j++) {
        const d = dist(strip[i], strip[j]);
        if (d < best[0]) best = [d, strip[i], strip[j]];
      }
    return best;
  }

  const [, a, b] = rec(px, py);
  return [a, b].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
}

const pts = [[1, 1], [-1, -1], [3, 4], [6, 1], [-1, -6], [-4, -3]];
const r = closest(pts);
console.log(`[(${r[0][0]}, ${r[0][1]}), (${r[1][0]}, ${r[1][1]})]`); // [(-1, -1), (1, 1)]
