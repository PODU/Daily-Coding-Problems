// Day 509: Fewest-insertion palindrome with lexicographically earliest result.
// Memoized interval DP building the actual string. Time O(n^3), Space O(n^2).
function solve(s) {
  const n = s.length;
  if (n === 0) return "";
  const memo = new Map();

  function build(i, j) {
    if (i > j) return "";
    if (i === j) return s[i];
    const key = i * n + j;
    if (memo.has(key)) return memo.get(key);
    let res;
    if (s[i] === s[j]) {
      res = s[i] + build(i + 1, j - 1) + s[j];
    } else {
      const a = s[i] + build(i + 1, j) + s[i];
      const b = s[j] + build(i, j - 1) + s[j];
      if (a.length < b.length) res = a;
      else if (b.length < a.length) res = b;
      else res = a <= b ? a : b;
    }
    memo.set(key, res);
    return res;
  }

  return build(0, n - 1);
}

console.log(solve("race"));
console.log(solve("google"));
