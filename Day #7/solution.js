// Decode ways: DP where ways[i] depends on 1-digit and valid 2-digit suffixes.
// Time: O(n), Space: O(1).
function numDecodings(s) {
  if (s.length === 0 || s[0] === "0") return 0;
  let prev2 = 1, prev1 = 1;
  for (let i = 1; i < s.length; i++) {
    let cur = 0;
    if (s[i] !== "0") cur += prev1;
    const two = parseInt(s.slice(i - 1, i + 1), 10);
    if (two >= 10 && two <= 26) cur += prev2;
    prev2 = prev1;
    prev1 = cur;
  }
  return prev1;
}

console.log(numDecodings("111"));
