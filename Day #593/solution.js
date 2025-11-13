// Day 593: Count buildings with a view of the setting sun (west).
// Array is east->west (index 0 = east). A building sees the sunset iff it is
// taller than every building further west (higher index). Single right-to-left pass.
// Time: O(n), Space: O(1).
function countSunsetViews(heights) {
  let count = 0, maxSeen = -Infinity;
  for (let i = heights.length - 1; i >= 0; i--) {
    if (heights[i] > maxSeen) { count++; maxSeen = heights[i]; }
  }
  return count;
}

console.log(countSunsetViews([3, 7, 8, 3, 6, 1])); // 3
