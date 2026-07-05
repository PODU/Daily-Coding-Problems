// Trapping Rain Water: two-pointer sweep tracking left/right running maxima.
// Time: O(N), Space: O(1).
function trap(h) {
  let l = 0, r = h.length - 1;
  let leftMax = 0, rightMax = 0;
  let water = 0;
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

console.log(trap([2, 1, 2]));
console.log(trap([3, 0, 1, 3, 0, 5]));
