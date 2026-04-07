// Square root of a real number via Newton's method: x <- (x + n/x)/2.
// Quadratic convergence -> O(log(1/eps)) iterations, O(1) space.
'use strict';

function mySqrt(n) {
  if (n < 0) throw new Error("negative");
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 100; i++) {
    const nx = 0.5 * (x + n / x);
    if (Math.abs(nx - x) < 1e-15) break;
    x = nx;
  }
  return x;
}

const r = mySqrt(9);
console.log(Math.abs(r - Math.round(r)) < 1e-9 ? Math.round(r) : r); // 3
