// Min flips so all x's precede all y's. Greedy: res=min(res+1, yCount).
// Time O(n), Space O(1).
function minFlips(s) {
  let res = 0, yCount = 0;
  for (const ch of s) {
    if (ch === 'y') yCount++;
    else res = Math.min(res + 1, yCount);
  }
  return res;
}

console.log(minFlips("xyxxxyxyy"));
