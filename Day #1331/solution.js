// Day 1331: Does any pair of rectangles overlap (full containment counts; edge-touching does not)?
// Convert top_left+dimensions to [xmin,xmax,ymin,ymax]; pairwise strict-interval overlap test. O(n^2).

function make(tlx, tly, w, h) {
  return [tlx, tlx + w, tly - h, tly]; // xmin, xmax, ymin, ymax
}

function overlap(a, b) {
  return a[0] < b[1] && b[0] < a[1] && a[2] < b[3] && b[2] < a[3];
}

function anyOverlap(rects) {
  for (let i = 0; i < rects.length; i++)
    for (let j = i + 1; j < rects.length; j++)
      if (overlap(rects[i], rects[j])) return true;
  return false;
}

const rects = [
  make(1, 4, 3, 3),
  make(-1, 3, 2, 1),
  make(0, 5, 4, 3),
];
console.log(anyOverlap(rects)); // true
