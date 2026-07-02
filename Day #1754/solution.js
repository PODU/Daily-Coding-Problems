// Day 1754: All strobogrammatic numbers with N digits.
// Build from middle outward placing rotatable pairs; skip leading 0 for outer layer.
// Time O(output size), space O(N) recursion depth.
'use strict';

const PAIRS = [['0', '0'], ['1', '1'], ['6', '9'], ['8', '8'], ['9', '6']];

function build(n, total) {
  if (n === 0) return [''];
  if (n === 1) return ['0', '1', '8'];
  const inner = build(n - 2, total);
  const res = [];
  for (const s of inner) {
    for (const [a, b] of PAIRS) {
      if (a === '0' && n === total) continue; // no leading zero
      res.push(a + s + b);
    }
  }
  return res;
}

function strobogrammatic(n) {
  return build(n, n);
}

for (const n of [2, 3]) {
  console.log(`N=${n}: [${strobogrammatic(n).join(', ')}]`);
}
