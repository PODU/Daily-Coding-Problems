// Day 1729: Count buildings with a sunset (west) view.
// Single right-to-left pass tracking max height seen to the west; a building is
// visible iff strictly taller than all to its west. Time: O(n). Space: O(1).

function countSunsetViews(heights) {
  let count = 0;
  let maxWest = 0;
  // Scan from the west end (rightmost index) toward the east.
  for (let i = heights.length - 1; i >= 0; i--) {
    if (heights[i] > maxWest) {
      count++;
      maxWest = heights[i];
    }
  }
  return count;
}

function main() {
  const heights = [3, 7, 8, 3, 6, 1]; // east -> west
  console.log(countSunsetViews(heights)); // 1, 6, 8 visible => 3
}

main();
