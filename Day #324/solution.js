// Assign mice to holes minimizing max distance: sort both, pair i-th, answer = max|mice[i]-holes[i]|.
// Time: O(N log N), Space: O(1) extra.
function minMaxDistance(mice, holes) {
  mice = [...mice].sort((a, b) => a - b);
  holes = [...holes].sort((a, b) => a - b);
  let ans = 0;
  for (let i = 0; i < mice.length; i++)
    ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
  return ans;
}

const mice = [1, 4, 9, 15];
const holes = [10, -5, 0, 16];
console.log(minMaxDistance(mice, holes));
