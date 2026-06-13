// Shortest palindrome by insertions, lexicographically earliest: memoized DP on
// (i,j) building best palindrome for s[i..j]. Time O(n^2) states, Space O(n^2).
"use strict";

function shortestPalindrome(s) {
  const n = s.length;
  const memo = new Map();
  function solve(i, j) {
    if (i > j) return "";
    if (i === j) return s[i];
    const key = i * n + j;
    if (memo.has(key)) return memo.get(key);
    let res;
    if (s[i] === s[j]) {
      res = s[i] + solve(i + 1, j - 1) + s[i];
    } else {
      const opt1 = s[i] + solve(i + 1, j) + s[i];
      const opt2 = s[j] + solve(i, j - 1) + s[j];
      if (opt1.length < opt2.length) res = opt1;
      else if (opt2.length < opt1.length) res = opt2;
      else res = opt1 <= opt2 ? opt1 : opt2;
    }
    memo.set(key, res);
    return res;
  }
  return solve(0, n - 1);
}

console.log(shortestPalindrome("race"));
console.log(shortestPalindrome("google"));
