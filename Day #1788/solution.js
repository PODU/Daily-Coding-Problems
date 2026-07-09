// k nearest points: stable sort by squared Euclidean distance, take first k.
// Time O(N log N), Space O(N).
function kNearest(points, central, k) {
  const [cx, cy] = central;
  const idx = points.map((_, i) => i);
  idx.sort((a, b) => {
    const da = (points[a][0]-cx)**2 + (points[a][1]-cy)**2;
    const db = (points[b][0]-cx)**2 + (points[b][1]-cy)**2;
    return da - db;
  });
  return idx.slice(0, k).map(i => points[i]);
}

const points = [[0,0],[5,4],[3,1]];
const res = kNearest(points, [1,2], 2);
console.log("[" + res.map(p => `(${p[0]}, ${p[1]})`).join(", ") + "]");
