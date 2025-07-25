// Trapping rain water with two pointers.
// Time: O(n), Space: O(1).
function trap(h) {
  let l = 0, r = h.length - 1;
  let leftMax = 0, rightMax = 0, water = 0;
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

const heights = [3, 0, 1, 3, 0, 5];
console.log(trap(heights));
