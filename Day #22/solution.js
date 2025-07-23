// Word Break reconstruction: DP over positions with memoization using a word set.
// Time: O(n^2) substring checks (n = string length), Space: O(n) for memo + recursion.
function wordBreak(s, words) {
  const dict = new Set(words);
  const n = s.length;
  const memo = new Array(n + 1).fill(-2);

  function solve(i) {
    if (i === n) return true;
    if (memo[i] !== -2) return memo[i] !== -1;
    for (let j = i + 1; j <= n; j++) {
      if (dict.has(s.substring(i, j)) && solve(j)) {
        memo[i] = j - i;
        return true;
      }
    }
    memo[i] = -1;
    return false;
  }

  if (!solve(0)) return null;
  const res = [];
  for (let i = 0; i < n; ) {
    res.push(s.substring(i, i + memo[i]));
    i += memo[i];
  }
  return res;
}

const words = ['quick', 'brown', 'the', 'fox'];
const string = 'thequickbrownfox';
const res = wordBreak(string, words);
if (res === null) {
  console.log('None');
} else {
  console.log('[' + res.map((w) => `'${w}'`).join(', ') + ']');
}
