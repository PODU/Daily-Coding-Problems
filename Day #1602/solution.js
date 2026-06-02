// Min flips so all x precede all y. DP: at each char, flips = min(yCount, flips+1):
// flipping current 'y'->'x' costs all prior y's; flipping current 'x'->'y' costs flips+1. Time O(n), space O(1).
"use strict";

function minFlips(s) {
  let flips = 0, y = 0;
  for (const c of s) {
    if (c === "y") y++;
    else flips = Math.min(y, flips + 1);
  }
  return flips;
}

function main() {
  console.log(minFlips("xyxxxyxyy")); // 2
}

main();
