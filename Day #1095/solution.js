// Detect if any pair of axis-aligned rectangles overlap (containment counts). Time O(n^2), Space O(n).
function fromTopLeft(x, y, w, h) {
  // top_left corner; width grows right, height grows down
  return { minx: x, maxx: x + w, miny: y - h, maxy: y };
}

function overlap(a, b) {
  return a.minx < b.maxx && b.minx < a.maxx && a.miny < b.maxy && b.miny < a.maxy;
}

function anyOverlap(rects) {
  for (let i = 0; i < rects.length; i++)
    for (let j = i + 1; j < rects.length; j++)
      if (overlap(rects[i], rects[j])) return true;
  return false;
}

const rects = [
  fromTopLeft(1, 4, 3, 3),
  fromTopLeft(-1, 3, 2, 1),
  fromTopLeft(0, 5, 4, 3)];
console.log(anyOverlap(rects) ? "true" : "false");
