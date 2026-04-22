// Single pass from the west end (array right), tracking the tallest seen so far;
// a building has a view iff it is taller than everything to its west.
// Time O(n), Space O(1).

function countSunsetViews(h) {
  let count = 0, maxW = -Infinity;
  for (let i = h.length - 1; i >= 0; i--) {
    if (h[i] > maxW) { count++; maxW = h[i]; }
  }
  return count;
}

console.log(countSunsetViews([3, 7, 8, 3, 6, 1])); // 3
