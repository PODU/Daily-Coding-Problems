// 24 game (fixed order): interval recursion combining left/right reachable
// values with + - * / using doubles + epsilon.
// Time: O(n^3 * S^2), Space: O(n^2 * S). Here n=4.
'use strict';

function solve(a, l, r) {
  if (l === r) return [a[l]];
  const res = [];
  for (let m = l; m < r; m++) {
    const L = solve(a, l, m);
    const R = solve(a, m + 1, r);
    for (const x of L) for (const y of R) {
      res.push(x + y);
      res.push(x - y);
      res.push(x * y);
      if (Math.abs(y) > 1e-9) res.push(x / y);
    }
  }
  return res;
}

function can24(a) {
  return solve(a, 0, a.length - 1).some(v => Math.abs(v - 24.0) < 1e-6);
}

console.log(can24([5, 2, 7, 8]) ? 'True' : 'False');
