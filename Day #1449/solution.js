// Day 1449: Trapping Rain Water. Two-pointer sweep tracking running left/right
// maxima. Time O(n), Space O(1).
function trap(h) {
  let l = 0, r = h.length - 1, leftMax = 0, rightMax = 0, water = 0;
  while (l < r) {
    if (h[l] < h[r]) {
      leftMax = Math.max(leftMax, h[l]);
      water += leftMax - h[l];
      l++;
    } else {
      rightMax = Math.max(rightMax, h[r]);
      water += rightMax - h[r];
      r--;
    }
  }
  return water;
}

console.log(trap([2, 1, 2]));        // 1
console.log(trap([3, 0, 1, 3, 0, 5])); // 8
