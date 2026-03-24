// Min flips so all x before all y: single-pass DP. flips=min(flips+1, yCount) on 'x', yCount++ on 'y'. O(n) time, O(1) space.

function minFlips(s) {
  let flips = 0, yCount = 0;
  for (const c of s) {
    if (c === 'y') yCount++;
    else flips = Math.min(flips + 1, yCount);
  }
  return flips;
}

console.log(minFlips("xyxxxyxyy"));
