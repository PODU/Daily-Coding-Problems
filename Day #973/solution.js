// Day 973: Count ways to decode a digit string (a=1..z=26).
// Approach: DP, dp[i]=dp[i-1] if 1-digit valid + dp[i-2] if 2-digit valid. Time O(n), Space O(1).

function numDecodings(s) {
  if (!s || s[0] === '0') return 0;
  let prev2 = 1, prev1 = 1;
  for (let i = 1; i < s.length; i++) {
    let cur = 0;
    if (s[i] !== '0') cur += prev1;
    const two = parseInt(s.slice(i - 1, i + 1), 10);
    if (two >= 10 && two <= 26) cur += prev2;
    prev2 = prev1;
    prev1 = cur;
  }
  return prev1;
}

console.log(numDecodings('111')); // 3
