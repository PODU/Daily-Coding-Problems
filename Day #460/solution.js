// Day 460: Min flips so every 'x' precedes every 'y'.
// One-pass DP: dp = min(flip this x->y, flip all prior y->x). Time O(n), Space O(1).
function minFlips(s) {
  let dp = 0, y = 0;
  for (const c of s) {
    if (c === "y") y++;
    else dp = Math.min(dp + 1, y);
  }
  return dp;
}

console.log(minFlips("xyxxxyxyy")); // 2
