// Day 1581: Area of intersection of two axis-aligned rectangles.
// top_left=(x,y), dims=(w,h); x-range [x,x+w], y-range [y-h,y]. Overlap = clamped widths.
// Time: O(1); Space: O(1).
"use strict";

function intersectionArea(r1, r2) {
  const [x1, y1] = r1.top_left, [w1, h1] = r1.dimensions;
  const [x2, y2] = r2.top_left, [w2, h2] = r2.dimensions;
  const ow = Math.min(x1 + w1, x2 + w2) - Math.max(x1, x2);
  const oh = Math.min(y1, y2) - Math.max(y1 - h1, y2 - h2);
  if (ow <= 0 || oh <= 0) return 0;
  return ow * oh;
}

const a = { top_left: [1, 4], dimensions: [3, 3] };
const b = { top_left: [0, 5], dimensions: [4, 3] };
console.log(intersectionArea(a, b)); // 6
