// Day 362: Strobogrammatic numbers of N digits.
// Recursively build from outside in using rotatable digit pairs; skip leading 0.
// Time O(output size), Space O(N) recursion depth.
'use strict';

const PAIRS = [['0', '0'], ['1', '1'], ['6', '9'], ['8', '8'], ['9', '6']];

function build(n, total) {
  if (n === 0) return [''];
  if (n === 1) return ['0', '1', '8'];
  const inner = build(n - 2, total);
  const res = [];
  for (const s of inner)
    for (const [a, b] of PAIRS) {
      if (n === total && a === '0') continue;
      res.push(a + s + b);
    }
  return res;
}

function strobogrammatic(n) {
  return build(n, n);
}

const n = 2;
console.log(`N=${n}: ` + strobogrammatic(n).join(' '));
