// Day 1106: Count buildings (east->west) with a clear sunset view to the west.
// A building sees west if taller than all to its west; scan from west end, track max.
// Time: O(N) single pass. Space: O(1).
function sunsetViews(h) {
  let count = 0, maxSoFar = -Infinity;
  for (let i = h.length - 1; i >= 0; i--)
    if (h[i] > maxSoFar) { count++; maxSoFar = h[i]; }
  return count;
}

console.log(sunsetViews([3, 7, 8, 3, 6, 1])); // 3
