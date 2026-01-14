// 24 game, fixed order: recursively split the sequence at each position, combine left/right results with +,-,*,/ (eps for div). O(4^n) over splits; O(n) depth.
'use strict';

const EPS = 1e-6;

function solve(nums, lo, hi) {
  if (hi - lo === 1) return [nums[lo]];
  const res = [];
  for (let mid = lo + 1; mid < hi; mid++) {
    const L = solve(nums, lo, mid);
    const R = solve(nums, mid, hi);
    for (const a of L) for (const b of R) {
      res.push(a + b);
      res.push(a - b);
      res.push(a * b);
      if (Math.abs(b) > EPS) res.push(a / b);
    }
  }
  return res;
}

function canReach24(nums) {
  const vals = nums.map(Number);
  return solve(vals, 0, vals.length).some(v => Math.abs(v - 24.0) < EPS);
}

const input = [5, 2, 7, 8];
console.log(canReach24(input) ? "True" : "False");
