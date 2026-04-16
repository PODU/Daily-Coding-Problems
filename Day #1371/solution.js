// Assign mice to holes minimizing max distance: sort both, pair in order.
// Time O(n log n), Space O(1) extra.
function minLastMouse(mice, holes) {
  mice = [...mice].sort((a, b) => a - b);
  holes = [...holes].sort((a, b) => a - b);
  let ans = 0;
  for (let i = 0; i < mice.length; i++)
    ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
  return ans;
}

console.log(minLastMouse([1, 4, 9, 15], [10, -5, 0, 16]));
