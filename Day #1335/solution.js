// Day 1335: Count decodings of a digit string with a=1..z=26.
// DP: ways[i] += ways[i-1] if digit valid, += ways[i-2] if two-digit 10..26 valid. O(n) time, O(1) space.

function numDecodings(s) {
  const n = s.length;
  if (n === 0) return 0;
  let prev2 = 1, prev1 = s[0] !== "0" ? 1 : 0;
  for (let i = 1; i < n; i++) {
    let cur = 0;
    if (s[i] !== "0") cur += prev1;
    const two = parseInt(s.substring(i - 1, i + 1), 10);
    if (two >= 10 && two <= 26) cur += prev2;
    prev2 = prev1;
    prev1 = cur;
  }
  return prev1;
}

console.log(numDecodings("111")); // 3
