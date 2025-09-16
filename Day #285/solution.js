// Day 285: Count buildings (east->west) with a sunset (west) view.
// Single backward pass tracking running max. Time O(N), Space O(1).
function sunsetViews(h) {
  let count = 0, maxSoFar = -Infinity;
  for (let i = h.length - 1; i >= 0; i--) {
    if (h[i] > maxSoFar) {
      count++;
      maxSoFar = h[i];
    }
  }
  return count;
}

console.log(sunsetViews([3, 7, 8, 3, 6, 1])); // 3
