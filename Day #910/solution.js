// Minimum jumps to reach end: greedy BFS-by-levels tracking current reach and farthest reach.
// Bump jumps when current window ends. Time O(n), Space O(1).

function minJumps(nums) {
  const n = nums.length;
  if (n <= 1) return 0;
  let jumps = 0, curEnd = 0, farthest = 0;
  for (let i = 0; i < n - 1; i++) {
    farthest = Math.max(farthest, i + nums[i]);
    if (i === curEnd) { jumps++; curEnd = farthest; }
  }
  return jumps;
}

const nums = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9];
console.log(minJumps(nums));
