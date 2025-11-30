// Day 673: K nearest points to a center. Sort by squared distance.
// Time O(n log n), Space O(n). No sqrt needed for comparison.
function kNearest(pts, c, k) {
  const d2 = (p) => (p[0] - c[0]) ** 2 + (p[1] - c[1]) ** 2;
  return pts
    .map((p, i) => [d2(p), i, p])
    .sort((a, b) => a[0] - b[0] || a[1] - b[1])
    .slice(0, k)
    .map((x) => x[2]);
}

const pts = [[0, 0], [5, 4], [3, 1]];
const r = kNearest(pts, [1, 2], 2);
console.log("[" + r.map((p) => `(${p[0]}, ${p[1]})`).join(", ") + "]"); // [(0, 0), (3, 1)]
