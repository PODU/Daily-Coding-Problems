// Day 187: Do any two rectangles overlap (containment counts; edge-touching does not).
// Pairwise interior-overlap test. Time O(n^2), Space O(1). (Sweep line gives O(n log n).)
function overlap(a, b) {
  const [al, at] = a.top_left, [aw, ah] = a.dimensions;
  const [bl, bt] = b.top_left, [bw, bh] = b.dimensions;
  const ox = Math.min(al + aw, bl + bw) - Math.max(al, bl);
  const oy = Math.min(at, bt) - Math.max(at - ah, bt - bh);
  return ox > 0 && oy > 0;
}

function anyOverlap(rs) {
  for (let i = 0; i < rs.length; i++)
    for (let j = i + 1; j < rs.length; j++)
      if (overlap(rs[i], rs[j])) return true;
  return false;
}

const rs = [
  { top_left: [1, 4], dimensions: [3, 3] },
  { top_left: [-1, 3], dimensions: [2, 1] },
  { top_left: [0, 5], dimensions: [4, 3] },
];
console.log(anyOverlap(rs));
