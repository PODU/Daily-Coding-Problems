// Day 185: Area of intersection of two axis-aligned rectangles (top-left + width/height, y up).
// Overlap = max(0, dx) * max(0, dy). Time O(1), Space O(1).
function intersectionArea(a, b) {
  const [al, at] = a.top_left, [aw, ah] = a.dimensions;
  const [bl, bt] = b.top_left, [bw, bh] = b.dimensions;
  const ox = Math.min(al + aw, bl + bw) - Math.max(al, bl);
  const oy = Math.min(at, bt) - Math.max(at - ah, bt - bh);
  if (ox <= 0 || oy <= 0) return 0;
  return ox * oy;
}

const a = { top_left: [1, 4], dimensions: [3, 3] };
const b = { top_left: [0, 5], dimensions: [4, 3] };
console.log(intersectionArea(a, b));
