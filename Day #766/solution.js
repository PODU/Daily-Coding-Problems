// Day 766: Minimum flips so all 'x' come before all 'y'.
// One-pass DP: flips = min(flips+1, countY). O(n) time, O(1) space.
function minFlips(s) {
  let flips = 0, countY = 0;
  for (const c of s) {
    if (c === 'y') countY++;
    else flips = Math.min(flips + 1, countY);
  }
  return flips;
}

console.log(minFlips("xyxxxyxyy")); // 2
