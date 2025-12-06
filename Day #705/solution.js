// Day 705: Trapping rain water.
// Approach: two pointers tracking running left/right maxima; the smaller side is
// bounded so we can resolve it. Time O(N), Space O(1).
function trap(h) {
  let l = 0, r = h.length - 1, lmax = 0, rmax = 0, water = 0;
  while (l < r) {
    if (h[l] < h[r]) {
      lmax = Math.max(lmax, h[l]); water += lmax - h[l]; l++;
    } else {
      rmax = Math.max(rmax, h[r]); water += rmax - h[r]; r--;
    }
  }
  return water;
}

console.log(trap([2, 1, 2]));          // 1
console.log(trap([3, 0, 1, 3, 0, 5])); // 8
