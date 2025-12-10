// Day 725: Assign mice to holes minimizing the maximum distance any mouse moves.
// Approach: Sort both arrays, pair in order, answer = max |mice[i]-holes[i]|.
// Time: O(n log n), Space: O(1).

function minLastMouse(mice, holes) {
  mice = [...mice].sort((a, b) => a - b);
  holes = [...holes].sort((a, b) => a - b);
  let ans = 0;
  for (let i = 0; i < mice.length; i++)
    ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
  return ans;
}

console.log(minLastMouse([1, 4, 9, 15], [10, -5, 0, 16])); // 6
