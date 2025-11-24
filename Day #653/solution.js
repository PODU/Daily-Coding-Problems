// Brute-force all O(k^2) pairs; rectangles overlap iff their projections strictly overlap on both axes.
// Sweep-line O(k log k) is possible but k^2 is clear. Time O(k^2), space O(k).
function make(x, y, w, h) {
  // top_left (x,y), dims (w,h): x1=x, x2=x+w, y2=y(top), y1=y-h(bottom)
  return { x1: x, y1: y - h, x2: x + w, y2: y };
}

function overlap(a, b) {
  return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2;
}

function main() {
  const rects = [
    make(1, 4, 3, 3),   // R1
    make(-1, 3, 2, 1),  // R2
    make(0, 5, 4, 3),   // R3
  ];
  let any = false;
  for (let i = 0; i < rects.length && !any; i++) {
    for (let j = i + 1; j < rects.length; j++) {
      if (overlap(rects[i], rects[j])) { any = true; break; }
    }
  }
  console.log(any ? "true" : "false");
}

main();
