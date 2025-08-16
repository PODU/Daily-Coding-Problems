// Day 129: Square root of a real number via Newton's method.
// Quadratic convergence: O(log(1/eps)) iterations.
function mySqrt(n) {
  if (n < 0) return NaN;
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 100; i++) {
    const nx = 0.5 * (x + n / x);
    if (Math.abs(nx - x) < 1e-12) break;
    x = nx;
  }
  return x;
}

const n = 9;
const r = mySqrt(n);
console.log(Math.abs(r - Math.round(r)) < 1e-9 ? Math.round(r) : r);
