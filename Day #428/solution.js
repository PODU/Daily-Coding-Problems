// Day 428: Min cost to carve a strict pyramid (1,2,..,P,..,2,1); only lowering allowed.
// left[i]/right[i] cap each stone by distance from the ends; peak H = max min(left,right).
// A strict tent of peak H has sum H*H, so cost = sum(stones) - H*H. Time O(n), Space O(n).
function solve(stones) {
  const n = stones.length;
  const left = new Array(n), right = new Array(n);
  left[0] = Math.min(stones[0], 1);
  for (let i = 1; i < n; i++) left[i] = Math.min(stones[i], left[i - 1] + 1);
  right[n - 1] = Math.min(stones[n - 1], 1);
  for (let i = n - 2; i >= 0; i--) right[i] = Math.min(stones[i], right[i + 1] + 1);
  let H = 0, p = 0;
  for (let i = 0; i < n; i++) {
    const b = Math.min(left[i], right[i]);
    if (b > H) { H = b; p = i; }
  }
  const total = stones.reduce((a, b) => a + b, 0);
  const cost = total - H * H;
  const heights = [];
  for (let i = 0; i < n; i++) heights.push(Math.max(0, H - Math.abs(i - p)));
  return [cost, heights];
}

const [cost, heights] = solve([1, 1, 3, 3, 2, 1]);
console.log(`${cost}  (resulting in [${heights.join(", ")}])`);
