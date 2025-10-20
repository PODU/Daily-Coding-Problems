// Square root via Newton's method: x = (x + n/x)/2 until convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
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
if (Math.abs(r - Math.round(r)) < 1e-9) console.log(Math.round(r));
else console.log(r);
