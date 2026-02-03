// Square root via Newton's method: x = (x + n/x)/2, quadratic convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).

function mySqrt(n) {
  if (n < 0) return NaN;
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 100; i++) {
    const nx = 0.5 * (x + n / x);
    if (Math.abs(nx - x) < 1e-15) { x = nx; break; }
    x = nx;
  }
  return x;
}

function printResult(n) {
  const r = mySqrt(n);
  const ri = Math.round(r);
  if (Math.abs(r - ri) < 1e-9) console.log(ri);          // exact integer
  else console.log(r.toFixed(8));
}

printResult(9);   // -> 3
printResult(2);   // -> 1.41421356
