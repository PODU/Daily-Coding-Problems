// Day 1101: Assign mice to holes minimizing the maximum travel distance.
// Sort both, match in order; answer is max |mice[i]-holes[i]|.
// Time: O(N log N). Space: O(1).
function minLastMouse(mice, holes) {
  mice = [...mice].sort((a, b) => a - b);
  holes = [...holes].sort((a, b) => a - b);
  let ans = 0;
  for (let i = 0; i < mice.length; i++) ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
  return ans;
}

console.log(minLastMouse([1, 4, 9, 15], [10, -5, 0, 16])); // 6
