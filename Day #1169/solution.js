// K nearest points: keep k smallest by squared Euclidean distance, then sort
// the k results by (dist, original index) for deterministic output.
// Time: O(n log k) with a heap; here a simple sort-of-k for clarity. Space: O(k).
'use strict';

function kNearest(points, central, k) {
  const [cx, cy] = central;
  const dist2 = p => (p[0] - cx) ** 2 + (p[1] - cy) ** 2;

  // index list, partial-select k smallest then sort deterministically
  const idx = points.map((_, i) => i);
  idx.sort((a, b) => {
    const da = dist2(points[a]), db = dist2(points[b]);
    if (da !== db) return da - db;
    return a - b;
  });
  const chosen = idx.slice(0, k).sort((a, b) => {
    const da = dist2(points[a]), db = dist2(points[b]);
    if (da !== db) return da - db;
    return a - b;
  });
  return chosen.map(i => points[i]);
}

const points = [[0, 0], [5, 4], [3, 1]];
const central = [1, 2];
const k = 2;
const res = kNearest(points, central, k);
console.log('[' + res.map(p => `(${p[0]}, ${p[1]})`).join(', ') + ']');
