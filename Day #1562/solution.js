// Sort mice and holes, pair by index, answer = max |mice[i]-holes[i]|. Time O(n log n), Space O(1).
function assign(mice, holes) {
  mice = [...mice].sort((a, b) => a - b);
  holes = [...holes].sort((a, b) => a - b);
  let ans = 0;
  for (let i = 0; i < mice.length; i++) {
    ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
  }
  return ans;
}

function main() {
  const mice = [1, 4, 9, 15];
  const holes = [10, -5, 0, 16];
  console.log(assign(mice, holes));
}

main();
