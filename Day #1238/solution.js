// Fewest insertions for palindrome; lexicographically smallest among minima.
// Interval DP with memoized reconstruction. Time/Space O(n^2).
function makePalindrome(str) {
  const s = str;
  const n = s.length;
  const memo = new Map();
  const build = (i, j) => {
    if (i > j) return "";
    if (i === j) return s[i];
    const key = i * n + j;
    if (memo.has(key)) return memo.get(key);
    let res;
    if (s[i] === s[j]) {
      res = s[i] + build(i + 1, j - 1) + s[i];
    } else {
      const left = s[i] + build(i + 1, j) + s[i];
      const right = s[j] + build(i, j - 1) + s[j];
      if (left.length !== right.length) res = left.length < right.length ? left : right;
      else res = left <= right ? left : right;
    }
    memo.set(key, res);
    return res;
  };
  return build(0, n - 1);
}

console.log(makePalindrome("race"));
console.log(makePalindrome("google"));
